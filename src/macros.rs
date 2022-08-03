use paste::paste;

//#[macro_export]
macro_rules! single_response_type {
    ( $base:ident, $key:ident, $return:ident ) => {
        paste::paste! {
            #[derive(Debug, PartialEq, Serialize, Deserialize)]
            pub struct [< $return Response >] {
                pub [< $key:lower >]: $base,
            }

            impl SingleResponse for [< $return Response >] {
                type Inner = $base;

                fn inner(self) -> Self::Inner {
                    self.[< $key:lower >]
                }
            }
        }
    };
}

//#[macro_export]
macro_rules! plural_response_type {
    ( $base:ident, $key:ident, $return:ident ) => {
        paste::paste! {
            #[derive(Debug, PartialEq, Serialize, Deserialize)]
            pub struct [< $return Response >] {
                pub [< $key:lower >]: Vec<$base>,
            }

            impl SingleResponse for [< $return Response >] {
                type Inner = Vec<$base>;

                fn inner(self) -> Self::Inner {
                    self.[< $key:lower >]
                }
            }
        }
    };
}

//#[macro_export]
#[allow(unused_macros)]
macro_rules! plural_request_type {
    ( $base:ident, $key:ident ) => {
        paste::paste! {
            #[derive(Debug, PartialEq, Serialize, Deserialize)]
            pub struct [< $base Body >] {
                pub [< $key:lower >]: Vec<$base>,
            }
        }
    };
}

//#[macro_export]
macro_rules! list_response_type {
    ( $base:ident, $name:ident, $key:ident, $return:ident ) => {
        paste::paste! {
            #[derive(Debug, PartialEq, Serialize, Deserialize)]
            pub struct [< $name Response >] {
                pub offset: Option<usize>,
                pub more: Option<bool>,
                pub limit: Option<usize>,
                pub total: Option<u64>,
                pub [< $key >]: Vec<$return>,
            }

            impl PaginatedResponse<crate::praiya::PaginatedLegacyPosition> for [< $name Response >] {
                type Inner = Vec<$return>;
                type Cursor = usize;

                fn get_pos(&self) -> Self::Cursor {
                    self.offset.unwrap_or(1)
                }

                fn get_limit(&self) -> usize {
                    self.limit.unwrap_or(100)
                }

                fn inner(self) -> Self::Inner {
                    self.[< $key >]
                }

                fn has_more(&self) -> bool {
                    self.offset.is_some() && self.limit.is_some() && self.more.unwrap_or(false)
                }

                fn to_cursor(&self) -> crate::praiya::PaginatedLegacyPosition {
                    crate::praiya::PaginatedLegacyPosition {
                        offset: self.get_pos(),
                        has_more: self.has_more(),
                        limit: self.get_limit(),
                    }
                }
            }
        }
    };
}
