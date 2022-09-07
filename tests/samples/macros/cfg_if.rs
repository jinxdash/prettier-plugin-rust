[
    cfg_if! { if #[cfg(def)]{ use std; 0 } },
	cfg_if! { if #[cfg(def)]{ use std; 0 } else { 1 } },
    cfg_if! { if #[cfg(abc)]{ 0 } else { 1 } },
    cfg_if! { if #[cfg(abc)]{ 0 } else if #[cfg(def)] { 1 } },
    cfg_if! { if #[cfg(abc)]{ 0 } else if #[cfg(def)] { 1 } else { 0 } },

    cfg_if! {},
    cfg_if! { // comment
        if #[cfg(abc)] { 0 } },
    cfg_if! { 
        if #[cfg(abc)] { 
            /// comment
            /// comment
            /// comment
            struct A{
                //! comment
                a: u8
            }
         } },
    cfg_if! { 
        if #[cfg(abc)] { 
            struct A{
                //! comment
                // comment
                /// comment
                a: u8
            }
         } },
    cfg_if! { 
        if #[cfg(abc)] { 
            struct A{
                /// comment
                a: u8
            }
         }
    },
    cfg_if! { 
        if #[cfg(abc)] { 
            struct A{
                //! comment
            }
         }
    },
    cfg_if! { 
        if #[cfg(abc)] { 
            struct A{
                /// comment
            }
         }
    },
    cfg_if! { if true{} }
]