use darling::FromMeta;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, Visibility};

#[derive(Debug, Default, FromMeta)]
pub(crate) struct UnstableAttribute {
    /// The name of the feature that enables the unstable API.
    ///
    /// If not specified, the item will instead be guarded by a catch-all `unstable` feature.
    feature: Option<String>,

    /// A link or reference to a tracking issue for the unstable feature.
    ///
    /// This will be included in the item's documentation.
    issue: Option<String>,
}

impl UnstableAttribute {
    fn crate_feature_name(&self) -> String {
        if let Some(name) = self.feature.as_deref() {
            format!("unstable-{}", name)
        } else {
            String::from("unstable")
        }
    }

    pub(crate) fn expand(&self, mut item: impl ItemLike + ToTokens + Clone) -> TokenStream {
        // We only care about public items.
        if item.is_public() {
            let feature_name = self.crate_feature_name();

            if let Some(issue) = &self.issue {
                let doc_addendum = format!(
                    "\n\
                    # Availability\n\
                    \n\
                    **This API is marked as unstable** and is only available when \
                    the `{}` crate feature is enabled. This comes with no stability \
                    guarantees, and could be changed or removed at any time.\
                    \n\
                    The tracking issue is: `{}`\
                ",
                    feature_name, issue
                );
                item.push_attr(parse_quote! {
                    #[doc = #doc_addendum]
                });
            } else {
                let doc_addendum = format!(
                    "\n\
                    # Availability\n\
                    \n\
                    **This API is marked as unstable** and is only available when \
                    the `{}` crate feature is enabled. This comes with no stability \
                    guarantees, and could be changed or removed at any time.\
                ",
                    feature_name
                );
                item.push_attr(parse_quote! {
                    #[doc = #doc_addendum]
                });
            }

            let mut hidden_item = item.clone();
            hidden_item.set_visibility(parse_quote! {
                pub(crate)
            });

            TokenStream::from(quote! {
                #[cfg(feature = #feature_name)]
                #item

                #[cfg(not(feature = #feature_name))]
                #[allow(dead_code)]
                #hidden_item
            })
        } else {
            item.into_token_stream().into()
        }
    }
}

pub(crate) trait ItemLike {
    #[allow(unused)]
    fn attrs(&self) -> &[syn::Attribute];

    fn push_attr(&mut self, attr: syn::Attribute);

    fn visibility(&self) -> &Visibility;

    fn set_visibility(&mut self, visibility: Visibility);

    fn is_public(&self) -> bool {
        matches!(self.visibility(), Visibility::Public(_))
    }
}

macro_rules! impl_has_visibility {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl ItemLike for $ty {
                fn attrs(&self) -> &[syn::Attribute] {
                    &self.attrs
                }

                fn push_attr(&mut self, attr: syn::Attribute) {
                    self.attrs.push(attr);
                }

                fn visibility(&self) -> &Visibility {
                    &self.vis
                }

                fn set_visibility(&mut self, visibility: Visibility) {
                    self.vis = visibility;
                }
            }
        )*
    };
}

impl_has_visibility!(
    syn::ItemType,
    syn::ItemEnum,
    syn::ItemFn,
    syn::ItemMod,
    syn::ItemTrait,
    syn::ItemConst,
    syn::ItemStatic,
    syn::ItemUse,
);

impl ItemLike for syn::ItemStruct {
    fn attrs(&self) -> &[syn::Attribute] {
        &self.attrs
    }

    fn push_attr(&mut self, attr: syn::Attribute) {
        self.attrs.push(attr);
    }

    fn visibility(&self) -> &Visibility {
        &self.vis
    }

    fn set_visibility(&mut self, visibility: Visibility) {
        // Also constrain visibility of all fields to be at most the given
        // item visibility.
        self.fields
            .iter_mut()
            .filter(|field| matches!(&field.vis, Visibility::Public(_)))
            .for_each(|field| field.vis = visibility.clone());

        self.vis = visibility;
    }
}
