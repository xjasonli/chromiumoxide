
macro_rules! remote_object_parse_accessor {
    (
        #attr [$($attr:tt)*],
        #mode.get $($get:ident)?,
        #mode.set $($set:ident)?,
        #name $name:ident,
        #name.symbol $($symbol:ident)?,
        #name.normal $($normal:ident)?,
        #type $type:tt,
        $($body:tt)*
    ) => {
        remote_object_parse_accessor_attr(
            remote_object_parse_accessor_kind(
                remote_object_parse_generic(
                    remote_object_parse_argument(
                        remote_object_parse_return_where_body(
                        )
                    ),
                    remote_object_parse_accessor_body,
                    $($body)*
                )
                remote_object_parse_accessor_set(
                    remote_object_parse_accessor_body(
                    )
                ),
                #attr [$($attr)*],
                #mode.get $($get)?,
                #mode.set $($set)?,
                #name.symbol $($symbol)?,
                #name.normal $($normal)?,
            ),
            #name $name,
            $($body)*
        )
    };
}

macro_rules! remote_object_parse_accessor_attr {
    (
        $callback:ident($($forward:tt)*),
        #name $name:ident,
        $($rest:tt)*
    ) => {
        remote_object_parse_accessor_attr!{@parse
            #callback $callback($($forward)*),
            #attr [],
            #name $name,
            #name.suffix ,
            $($rest)*
        }
    };

    (@parse
        #callback $callback:ident($($forward:tt)*),
        #attr [$($attr:tt)*],
        #name $name:ident,
        #name.suffix ,
        #[rename = + $suffix:ident]
        $($rest:tt)*
    ) => {
        remote_object_parse_accessor_attr!{@parse
            #callback $callback($($forward)*),
            #attr [$($attr)*],
            #name $name,
            #name.suffix $suffix,
            $($rest)*
        }
    };

    (@parse
        #callback $callback:ident($($forward:tt)*),
        #attr [$($attr:tt)*],
        #name $name:ident,
        #name.suffix $($suffix:ident)?,
        #[$meta:meta]
        $($rest:tt)*
    ) => {
        remote_object_parse_accessor_attr!{@parse
            #callback $callback($($forward)*),
            #attr [$($attr)* #[$meta]],
            #name $name,
            #name.suffix $($suffix)?,
            $($rest)*
        }
    };

    (@parse
        #callback $callback:ident($($forward:tt)*),
        #attr [$($attr:tt)*],
        #name $name:ident,
        #name.suffix $($suffix:ident)?,
        $($rest:tt)*
    ) => {
        paste::paste!{
            remote_object_parse_accessor_attr!{@callback
                #callback $callback($($forward)*),
                #attr [$($attr)*],
                #name [< $name $( _ $suffix:snake )? >],
                $($rest)*
            }
        }
    };

    (@callback
        #callback $callback:ident($($forward:tt)*),
        #attr [$($attr:tt)*],
        #name $name:ident,
        $($rest:tt)*
    ) => {
        $callback!{
            $($forward)*
            #attr.accessor [$($attr)*],
            #name $name,
            $($rest)*
        }
    };
}

macro_rules! remote_object_parse_accessor_kind {
    (
        $callback_get:ident($($forward_get:tt)*),
        $callback_set:ident($($forward_set:tt)*),
        #attr [$($attr:tt)*],
        #mode.get $($get:ident)?,
        #mode.set $($set:ident)?,
        #name.symbol $($symbol:ident)?,
        #name.normal $($normal:ident)?,
        #attr.accessor [$($attr_accessor:tt)*],
        #name $name:ident,
        $($rest:tt)*
    ) => {
        remote_object_parse_accessor_kind!{@parse
            #callback_get $callback_get(
                $($forward_get)*
                #attr [$($attr)*],
                #attr.accessor [$($attr_accessor)*],
                #name.symbol $($symbol)?,
                #name.normal $($normal)?,
                #name $name,
            ),
            #callback_set $callback_set($($forward_set)*),
            #mode.get $($get)?,
            #mode.set $($set)?,
            $($rest)*
        }
    };

    (@parse
        #callback_get $callback_get:ident($($forward_get:tt)*),
        #callback_set $callback_set:ident($($forward_set:tt)*),
        #mode.get get,
        #mode.set $($set:ident)?,
        get
        $($rest:tt)*
    ) => {
        $callback_get!{$($forward_get)*}
    };

    (@parse
        #callback_get $callback_get:ident($($forward_get:tt)*),
        #callback_set $callback_set:ident($($forward_set:tt)*),
        #mode.get $($get:ident)?,
        #mode.set set,
        set
        $($rest:tt)*
    ) => {
        $callback_set!{$($forward_set)*}
    };
}

macro_rules! remote_object_parse_accessor_get {
    (
        $callback:ident($($forward:tt)*),
        $($rest:tt)*
    ) => {
        $callback!{
            $($forward)*
            $($rest)*
        }
    };
}

macro_rules! remote_object_parse_accessor_set {
    (
    ) => {

    };
}

pub(crate) use {
    remote_object_parse_accessor,
    remote_object_parse_accessor_attr,
    remote_object_parse_accessor_kind,
    remote_object_parse_accessor_get,
    remote_object_parse_accessor_set,
};

