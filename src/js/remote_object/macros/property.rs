macro_rules! remote_object_parse_property {
    (
        $($rest:tt)*
    ) => {
        remote_object_parse_property_attr!{
            remote_object_parse_property_name!(
                remote_object_parse_property_kind!(
                    remote_object_parse_field(
                        remote_object_parse_accessor(),
                        remote_object_parse_property(),
                    ),
                    remote_object_parse_method_outer(),
                ),
            ),
            $($rest)*
        }
    };
}

macro_rules! remote_object_parse_property_attr {
    (
        $callback:ident($($forward:tt)*),
        $($rest:tt)*
    ) => {
        remote_object_parse_property_attr!{@step
            #callback $callback($($forward)*),
            #attr [],
            #mode.get get,
            #mode.set set,
            #name.rename ,
            #name.suffix ,
            $($rest)*
        }
    };

    // [rename = $name]
    (@step
        #callback $callback:ident($($forward:tt)*),
        #attr [$($attr:tt)*],
        #mode.get $($get:ident)?,
        #mode.set $($set:ident)?,
        #name.rename $($rename:ident)?,
        #name.suffix $($suffix:ident)?,
        #[rename = $name:ident]
        $($rest:tt)*
    ) => {
        paste::paste!{
            remote_object_parse_property_attr!{@step
                #callback $callback($($forward)*),
                #attr [$($attr)*],
                #mode.get $($get)?,
                #mode.set $($set)?,
                #name.rename [< $name:snake >],
                #name.suffix ,
                $($rest:tt)*
            }
        }
    };
    // [rename = + $name]
    (@step
        #callback $callback:ident($($forward:tt)*),
        #attr [$($attr:tt)*],
        #mode.get $($get:ident)?,
        #mode.set $($set:ident)?,
        #name.rename $($rename:ident)?,
        #name.suffix $($suffix:ident)?,
        #[rename = + $name:ident]
        $($rest:tt)*
    ) => {
        paste::paste!{
            remote_object_parse_property_attr!{@step
                #callback $callback($($forward)*),
                #attr [$($attr)*],
                #mode.get $($get)?,
                #mode.set $($set)?,
                #name.rename ,
                #name.suffix [< $name:snake >],
                $($rest:tt)*
            }
        }
    };
    // [readonly]
    (@step
        #callback $callback:ident($($forward:tt)*),
        #attr [$($attr:tt)*],
        #mode.get $($get:ident)?,
        #mode.set $($set:ident)?,
        #name.rename $($rename:ident)?,
        #name.suffix $($suffix:ident)?,
        #[readonly]
        $($rest:tt)*
    ) => {
        remote_object_parse_property_attr!{@step
            #callback $callback($($forward)*),
            #attr [$($attr)*],
            #mode.get $($get)?,
            #mode.set ,
            #name.rename $($rename)?,
            #name.suffix $($suffix)?,
            $($rest:tt)*
        }
    };
    // other attr
    (@step
        #callback $callback:ident($($forward:tt)*),
        #attr [$($attr:tt)*],
        #mode.get $($get:ident)?,
        #mode.set $($set:ident)?,
        #name.rename $($rename:ident)?,
        #name.suffix $($suffix:ident)?,
        #[$meta:meta]
        $($rest:tt)*
    ) => {
        remote_object_parse_property_attr!{@step
            #callback $callback($($forward)*),
            #attr [$($attr)* #[$meta]],
            #mode.get $($get)?,
            #mode.set $($set)?,
            #name.rename $($rename)?,
            #name.suffix $($suffix)?,
            $($rest:tt)*
        }
    };
    // attr ends
    (@step
        #callback $callback:ident($($forward:tt)*),
        #attr [$($attr:tt)*],
        #mode.get $($get:ident)?,
        #mode.set $($set:ident)?,
        #name.rename $($rename:ident)?,
        #name.suffix $($suffix:ident)?,
        $($rest:tt)*
    ) => {
        $callback!{
            $($forward)*
            #attr [$($attr)*],
            #mode.get $($get)?,
            #mode.set $($set)?,
            #name.rename $($rename)?,
            #name.suffix $($suffix)?,
            $($rest:tt)*
        }
    };
}

macro_rules! remote_object_parse_property_name {
    (
        $callback:ident($($forward:tt)*),
        #attr [$($attr:tt)*],
        #mode.get $($get:ident)?,
        #mode.set $($set:ident)?,
        #name.rename $($rename:ident)?,
        #name.suffix $($suffix:ident)?,
        $($rest:tt)*
    ) => {
        remote_object_parse_property_name!{@parse
            #callback $callback(
                $($forward)*
                #attr [$($attr)*],
                #mode.get $($get)?,
                #mode.set $($set)?,
            ),
            #name.rename $($rename)?,
            #name.suffix $($suffix)?,
            $($rest)*
        }
    };

    (@parse
        #callback $callback:ident($($forward:tt)*),
        #name.rename $($rename:ident)?,
        #name.suffix $($suffix:ident)?,
        Symbol.$symbol:ident
        $($rest:tt)*
    ) => {
        paste::paste!{
            remote_object_parse_property_name!{@rename
                $callback($($forward)*),
                #name.rename $($rename)?,
                #name.suffix $($suffix)?,
                #name.origin [< symbol_ $symbol:snake >],
                #name.symbol $symbol,
                #name.normal ,
                $($rest)*
            }
        }
    };

    (@parse
        #callback $callback:ident($($forward:tt)*),
        #name.rename $($rename:ident)?,
        #name.suffix $($suffix:ident)?,
        $name:ident
        $($rest:tt)*
    ) => {
        paste::paste!{
            remote_object_parse_property_name!{@rename
                $callback($($forward)*),
                #name.rename $($rename)?,
                #name.suffix $($suffix)?,
                #name.origin [< $name:snake >],
                #name.symbol ,
                #name.normal $name,
                $($rest)*
            }
        }
    };

    // no #[rename]
    (@rename
        #callback $callback:ident($($forward:tt)*),
        #name.rename ,
        #name.suffix ,
        #name.origin $origin:ident,
        #name.symbol $($symbol:ident)?,
        #name.normal $($normal:ident)?,
        $($rest:tt)*
    ) => {
        $callback!{
            $($forward)*
            #name $origin
            #name.symbol $($symbol)?,
            #name.normal $($normal)?,
            $($rest)*
        }
    };

    // #[rename = $rename]
    (@rename
        #callback $callback:ident($($forward:tt)*),
        #name.rename $rename:ident,
        #name.suffix ,
        #name.origin $origin:ident,
        #name.symbol $($symbol:ident)?,
        #name.normal $($normal:ident)?,
        $($rest:tt)*
    ) => {
        $callback!{
            $($forward)*
            #name $rename
            #name.symbol $($symbol)?,
            #name.normal $($normal)?,
            $($rest:tt)*
        }
    };

    // #[rename = + $suffix]
    (@rename
        #callback $callback:ident($($forward:tt)*),
        #name.rename ,
        #name.suffix $suffix:ident,
        #name.origin $origin:ident,
        #name.symbol $($symbol:ident)?,
        #name.normal $($normal:ident)?,
        $($rest:tt)*
    ) => {
        paste::paste!{
            $callback!{
                $($forward)*
                #name [< $origin _ $suffix >],
                #name.symbol $($symbol)?,
                #name.normal $($normal)?,
                $($rest:tt)*
            }
        }
    };
}

macro_rules! remote_object_parse_property_kind {
    (
        $callback_field:ident($($forward_field:tt)*),
        $callback_method:ident($($forward_method:tt)*),
        #attr [$($attr:tt)*],
        #mode.get $($get:ident)?,
        #mode.set $($set:ident)?,
        #name $name:ident,
        #name.symbol $($symbol:ident)?,
        #name.normal $($normal:ident)?,
        $($rest:tt)*
    ) => {
        remote_object_parse_property_kind!{@field_or_method
            #callback_field(
                $($forward_field)*
                #attr [$($attr)*],
                #mode.get $($get)?,
                #mode.set $($set)?,
                #name $name,
                #name.symbol $($symbol)?,
                #name.normal $($normal)?,
            ),
            #callback_method(
                $($forward_method)*
                #attr [$($attr)*],
                #mode.get $($get)?,
                #mode.set $($set)?,
                #name $name,
                #name.symbol $($symbol)?,
                #name.normal $($normal)?,
            ),
            $($rest)*
        }
    };

    // field
    (@field_or_method
        #callback_field($($forward_field:tt)*),
        #callback_method($($forward_method:tt)*),
        :
        $($rest:tt)*
    ) => {
        $callback_field!{
            $($forward_field)*
            : $($rest)*
        }
    };

    // field with optional
    (@field_or_method
        #callback_field($($forward_field:tt)*),
        #callback_method($($forward_method:tt)*),
        ? :
        $($rest:tt)*
    ) => {
        $callback_field!{
            $($forward_field)*
            ? : $($rest)*
        }
    };

    // method
    (@field_or_method
        #callback_field($($forward_field:tt)*),
        #callback_method($($forward_method:tt)*),
        $($rest:tt)*
    ) => {
        $callback_method!{
            $($forward_method)*
            $($rest)*
        }
        //remote_object_parse_generic!{
        //    remote_object_parse_argument(
        //        remote_object_parse_return_where_body(
        //            remote_object_method(
        //                #attr [$($attr)*],
        //                #name $name,
        //                #name.symbol $($symbol)?,
        //                #name.normal $($normal)?,
        //            ),
        //            remote_object_parse_properties,
        //        ),
        //    ),
        //    $($rest:tt)*
        //}
    };
}
