
macro_rules! remote_object_method {
    (
        // from remote_object_parse_properties
        #attr [$($attr:tt)*],
        #name $name:ident,
        #name.symbol $($symbol:ident)?,
        #name.normal $($normal:ident)?,

        // from remote_object_parse_generic
        #generic [$($generic:tt)*],

        // from remote_object_parse_argument
        #argument.lead ($($lead_name:ident: $lead_type:ty),*),
        #argument.last ($($last_name:ident: $last_type:ty [$($last_spread:ident)?])?),
        #argument.js ($($js:tt)*),

        // from remote_object_parse_return_where_body
        #return $return:tt,
        #where [$($where:tt)*],
        #body $( { $($body:tt)* } )?,
    ) => {
        
    };
}

pub(crate) use remote_object_method;
