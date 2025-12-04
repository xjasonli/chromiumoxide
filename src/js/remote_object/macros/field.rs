
macro_rules! remote_object_parse_field {
    (
        $callback_body:ident($($forward_body:tt)*),
        $callback_rest:ident($($forward_rest:tt)*),
        #attr [$($attr:tt)*],
        #mode [$($mode:ident),*],
        #name $name:ident,
        #name.symbol $($symbol:ident)?,
        #name.normal $($normal:ident)?,
        $($rest:tt)*
    ) => {
        remote_object_parse_field!{@parse
            #callback.body $callback_body(
                $($forward_body)*
                #attr [$($attr)*],
                #mode [$($mode),*],
                #name $name,
                #name.symbol $($symbol)?,
                #name.normal $($normal)?,
            ),
            #callback.rest $callback_rest($($forward_rest)*),
            $($rest)*
        }
    };

    // field without body
    (@parse
        #callback.body $callback_body:ident($($forward_body:tt)*),
        #callback.rest $callback_rest:ident($($forward_rest:tt)*),
        : $type:ty;
        $($rest:tt)*
    ) => {
        remote_object_parse_field!{@callback
            #callback.body $callback_body($($forward_body)*),
            #callback.rest $callback_rest($($forward_rest)*),
            #type $type,
            #body {},
            $($rest)*
        }
    };

    // field with body
    (@parse
        #callback.body $callback_body:ident($($forward_body:tt)*),
        #callback.rest $callback_rest:ident($($forward_rest:tt)*),
        : $type:ty {
            $($body:tt)*
        }
        $($rest:tt)*
    ) => {
        remote_object_parse_field!{@callback
            #callback.body $callback_body($($forward_body)*),
            #callback.rest $callback_rest($($forward_rest)*),
            #type $type,
            #body { $($body)* },
            $($rest)*
        }
    };

    // optional field without body
    (@parse
        #callback.body $callback_body:ident($($forward_body:tt)*),
        #callback.rest $callback_rest:ident($($forward_rest:tt)*),
        ? : $type:ty;
        $($rest:tt)*
    ) => {
        remote_object_parse_field!{@callback
            #callback.body $callback_body($($forward_body)*),
            #callback.rest $callback_rest($($forward_rest)*),
            #type Optional<$type>,
            #body {},
            $($rest)*
        }
    };

    // optional field with body
    (@parse
        #callback.body $callback_body:ident($($forward_body:tt)*),
        #callback.rest $callback_rest:ident($($forward_rest:tt)*),
        ? : $type:ty {
            $($body:tt)*
        }
        $($rest:tt)*
    ) => {
        remote_object_parse_field!{@callback
            #callback.body $callback_body($($forward_body)*),
            #callback.rest $callback_rest($($forward_rest)*),
            #type Optional<$type>,
            #body { $($body)* },
            $($rest)*
        }
    };

    (@callback
        #callback.body $callback_body:ident($($forward_body:tt)*),
        #callback.rest $callback_rest:ident($($forward_rest:tt)*),
        #type $type:tt,
        #body { $($body:tt)* },
        $($rest:tt)*
    ) => {
        $callback_body!{
            $($forward_body)*
            #type $type,
            $($body)*
        }
        $callback_rest!{
            $($forward_rest)*
            $($rest)*
        }
    };
}
