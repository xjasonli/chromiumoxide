
macro_rules! remote_object_parse_generic {
    (
        // callback macro
        $callback:ident($($forward:tt)*),
        // tokens starts with `<`
        <
        $($rest:tt)*
    ) => {
        remote_object_parse_generic!{@find_end
            #callback $callback($($forward)*),
            #generic [<],
            $($rest:tt)*
        }
    };

    (
        // callback macro
        $callback:ident($($forward:tt)*),
        // tokens without `<`
        $($rest:tt)*
    ) => {
        remote_object_parse_generic!{@callback
            #callback $callback($($forward)*),
            #generic [],
            $($rest:tt)*
        }
    };

    // find `>`
    (@find_end
        #callback $callback:ident($($forward:tt)*),
        #generic [$($generic:tt)*],
        >
        $($rest:tt)*
    ) => {
        remote_object_parse_generic!{@callback
            #callback $callback($($forward)*),
            #generic [$($generic)* >],
            $($rest:tt)*
        }
    };

    // find next token
    (@find_end
        #callback $callback:ident($($forward:tt)*),
        #generic [$($generic:tt)*],
        $next:tt
        $($rest:tt)*
    ) => {
        remote_object_parse_generic!{@end
            #callback $callback($($forward)*),
            #generic [$($generic)* $next],
            $($rest:tt)*
        }
    };

    // call callback macro
    (@callback
        #callback $callback:ident($($forward:tt)*),
        #generic [$($generic:tt)*],
        $($rest:tt)*
    ) => {
        $callback!{
            $($forward)*
            #generic [$($generic)*],
            $($rest:tt)*
        }
    };
}

macro_rules! remote_object_parse_argument {
    (
        // callback macro
        $callback:ident($($forward:tt)*),
        ($($argument:tt)*)
        $($rest:tt)*
    ) => {
        remote_object_parse_generic!{@collect
            #callback $callback($($forward)*),
            #trailing ($($rest)*),
            #lead (),
            #js (),
            $($argument)*
        }
    };

    // non-last argument
    (@collect
        #callback $callback:ident($($forward:tt)*),
        #trailing ($($trailing:tt)*),
        #lead ($($lead_name:ident: $lead_type:ty),*),
        #js ($($js:ident),*),
        $name:ident: $type:ty,
        $($rest:tt)+
    ) => {
        paste::paste!{
            remote_object_parse_argument!{@collect
                #callback $callback($($forward)*),
                #trailing ($($trailing)*),
                #lead ($($lead_name: $lead_type,)* [< $name:snake >]: $type),
                #js ($($js,)* $name),
                $($rest:tt)*
            }
        }
    };

    // non-last argument with optional
    (@collect
        #callback $callback:ident($($forward:tt)*),
        #trailing ($($trailing:tt)*),
        #lead ($($lead_name:tt: $lead_type:ty),*),
        #js ($($js:ident),*),
        $name:ident ? : $type:ty,
        $($rest:tt)+
    ) => {
        paste::paste!{
            remote_object_parse_argument!{@collect
                #callback $callback($($forward)*),
                #trailing ($($trailing)*),
                #lead ($($lead_name: $lead_type,)* [< $name:snake >]: Optional<$type>),
                #js ($($js,)* $name),
                $($rest:tt)*
            }
        }
    };

    // last argument
    (@collect
        #callback $callback:ident($($forward:tt)*),
        #trailing ($($trailing:tt)*),
        #lead ($($lead_name:tt: $lead_type:ty),*),
        #js ($($js:ident),*),
        $name:ident: $type:ty $(,)?
    ) => {
        paste::paste!{
            remote_object_parse_argument!{@callback
                #callback $callback($($forward)*),
                #trailing ($($trailing)*),
                #lead ($($lead_name: $lead_type),*),
                #last ([< $name:snake >]: $type []),
                #js ($($js,)* $name),
            }
        }
    };
    // last argument with optional
    (@collect
        #callback $callback:ident($($forward:tt)*),
        #trailing ($($trailing:tt)*),
        #lead ($($lead_name:tt: $lead_type:ty),*),
        #js ($($js:ident),*),
        $name:ident ? : $type:ty $(,)?
    ) => {
        remote_object_parse_argument!{@callback
            #callback $callback($($forward)*),
            #trailing ($($trailing)*),
            #lead ($($lead_name: $lead_type),*),
            #last ([< $name:snake >]: Optional<$type> []),
            #js ($($js,)* $name),
        }
    };
    // last argument with spread
    (@collect
        #callback $callback:ident($($forward:tt)*),
        #trailing ($($trailing:tt)*),
        #lead ($($lead_name:tt: $lead_type:ty),*),
        #js ($($js:ident),*),
        ... $name:ident : $type:ty $(,)?
    ) => {
        paste::paste!{
            remote_object_parse_argument!{@callback
                #callback $callback($($forward)*),
                #trailing ($($trailing)*),
                #lead ($($lead_name: $lead_type),*),
                #last ([< $name:snake >]: $type [_spread]),
                #js ($($js,)* ... $name),
            }
        }
    };
    // last argument with optional and spread
    (@collect
        #callback $callback:ident($($forward:tt)*),
        #trailing ($($trailing:tt)*),
        #lead ($($lead_name:tt: $lead_type:ty),*),
        #js ($($js:ident),*),
        ... $name:ident ? : $type:ty $(,)?
    ) => {
        paste::paste!{
            remote_object_parse_argument!{@callback
                #callback $callback($($forward)*),
                #trailing ($($trailing)*),
                #lead ($($lead_name: $lead_type),*),
                #last ([< $name:snake >]: Optional<$type> [_spread]),
                #js ($($js,)* ... $name),
            }
        }
    };

    (@callback
        #callback $callback:ident($($forward:tt)*),
        #trailing ($($trailing:tt)*),
        #lead ($($lead_name:tt: $lead_type:ty),*),
        #last ($($last_name:ident: $last_type:ty [$($last_spread:ident)?])?),
        #js ($($js:tt)*),
    ) => {
        $callback!{
            $($forward)*
            #argument.lead ($($lead_name: $lead_type),*),
            #argument.last ($($last_name: $last_type [$($last_spread)?])?),
            #argument.js ($($js:ident),*),
            $($trailing)*
        }
    };
}


macro_rules! remote_object_parse_return_where_body {
    // return + no where + no body
    (
        $callback_body:ident($($forward_body:tt)*),
        $callback_rest:ident($($forward_rest:tt)*),
        #return.default $default:ty,
        -> $ty:ty;
        $($rest:tt)*
    ) => {
        remote_object_parse_return_where_body!{@callback
            #callback.body $callback_body($($forward_body)*),
            #callback.rest $callback_rest($($forward_rest)*),
            #return Result<$default>,
            #where [],
            #body ,
            $($rest)*
        }
    };

    // return + no where + body
    (
        $callback_body:ident($($forward_body:tt)*),
        $callback_rest:ident($($forward_rest:tt)*),
        #return.default $default:ty,
        -> $ty:ty { $($body:tt)* }
        $($rest:tt)*
    ) => {
        remote_object_parse_return_where_body!{@callback
            #callback.body $callback_body($($forward_body)*),
            #callback.rest $callback_rest($($forward_rest)*),
            #return Result<$ty>,
            #where [],
            #body { $($body)? },
            $($rest)*
        }
    };

    // no return + no where + no body
    (
        $callback_body:ident($($forward_body:tt)*),
        $callback_rest:ident($($forward_rest:tt)*),
        #return.default $default:ty,
        ;
        $($rest:tt)*
    ) => {
        remote_object_parse_return_where_body!{@callback
            #callback.body $callback_body($($forward_body)*),
            #callback.rest $callback_rest($($forward_rest)*),
            #return Result<$default>,
            #where [],
            #body ,
            $($rest)*
        }
    };

    // no return + no where + body
    (
        $callback_body:ident($($forward_body:tt)*),
        $callback_rest:ident($($forward_rest:tt)*),
        #return.default $default:ty,
        { $($body:tt)* }
        $($rest:tt)*
    ) => {
        remote_object_parse_return_where_body!{@callback
            #callback.body $callback_body($($forward_body)*),
            #callback.rest $callback_rest($($forward_rest)*),
            #return Result<$default>,
            #where [],
            #body { $($body)? },
            $($rest)*
        }
    };

    // return + where
    (
        $callback_body:ident($($forward_body:tt)*),
        $callback_rest:ident($($forward_rest:tt)*),
        #return.default $default:ty,
        -> $ty:ty
        where
        $($rest:tt)*
    ) => {
        remote_object_parse_return_where_body!{@parse_where
            #callback.body $callback_body($($forward_body)*),
            #callback.rest $callback_rest($($forward_rest)*),
            #return Result<$ty>,
            #where [],
            $($rest:tt)*
        }
    };

    // no return + where
    (
        $callback_body:ident($($forward_body:tt)*),
        $callback_rest:ident($($forward_rest:tt)*),
        #return.default $default:ty,
        where
        $($rest:tt)*
    ) => {
        remote_object_parse_return_where_body!{@parse_where
            #callback.body $callback_body($($forward_body)*),
            #callback.rest $callback_rest($($forward_rest)*),
            #return Result<$default>,
            #where [],
            $($rest:tt)*
        }
    };

    // where ends with no body
    (@parse_where
        #callback_body $callback_body:ident($($forward_body:tt)*),
        #callback_rest $callback_rest:ident($($forward_rest:tt)*),
        #return $return:tt,
        #where [$($where:tt)*],
        ;
        $($rest:tt)*
    ) => {
        remote_object_parse_return_where_body!{@callback
            #callback.body $callback_body($($forward_body)*),
            #callback.rest $callback_rest($($forward_rest)*),
            #return $return,
            #where [$($where)*],
            #body ,
            $($rest:tt)*
        }
    };

    // where ends with body
    (@parse_where
        #callback_body $callback_body:ident($($forward_body:tt)*),
        #callback_rest $callback_rest:ident($($forward_rest:tt)*),
        #return $return:tt,
        #where [$($where:tt)*],
        { $($body:tt)* }
        $($rest:tt)*
    ) => {
        remote_object_parse_return_where_body!{@callback
            #callback.body $callback_body($($forward_body)*),
            #callback.rest $callback_rest($($forward_rest)*),
            #return $return,
            #where [$($where)*],
            #body { $($body)? },
            $($rest:tt)*
        }
    };

    // where item
    (@parse_where
        #callback_body $callback_body:ident($($forward_body:tt)*),
        #callback_rest $callback_rest:ident($($forward_rest:tt)*),
        #return $return:tt,
        #where [$($where:tt)*],
        $where_item:tt
        $($rest:tt)*
    ) => {
        remote_object_parse_return_where_body!{@parse_where
            #callback.body $callback_body($($forward_body)*),
            #callback.rest $callback_rest($($forward_rest)*),
            #return $return,
            #where [$($where)* $where_item],
            $($rest:tt)*
        }
    };

    (@callback
        #callback.body $callback_body:ident($($forward_body:tt)*),
        #callback.rest $callback_rest:ident($($forward_rest:tt)*),
        #return $return:tt,
        #where [$($where:tt)*],
        #body $( { $($body:tt)* } )?,
        $($rest:tt)*
    ) => {
        $callback_body!{
            $($forward_body)*
            #return $return,
            #where [$($where)*],
            #body $( { $($body)? } )?,
        }
        $callback_rest!{
            $($forward_rest)*
            $($rest)*
        }
    };
}

pub(crate) use {
    remote_object_parse_generic,
    remote_object_parse_argument,
    remote_object_parse_return_where_body,
};
