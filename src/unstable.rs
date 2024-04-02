use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::meta::ParseNestedMeta;
use syn::parse::Result;
use syn::{parse_quote, Visibility};

#[derive(Debug, Default)]
pub(crate) struct UnstableAttribute {
    feature: Option<String>,
    issue: Option<String>,
}

impl UnstableAttribute {
    pub(crate) fn parse(&mut self, meta: ParseNestedMeta) -> Result<()> {
        if meta.path.is_ident("feature") {
            match meta.value()?.parse()? {
                syn::Lit::Str(s) => self.feature = Some(s.value()),
                _ => panic!(),
            }
        } else if meta.path.is_ident("issue") {
            match meta.value()?.parse()? {
                syn::Lit::Str(s) => self.issue = Some(s.value()),
                _ => panic!(),
            }
        }
        Ok(())
    }

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
					guarantees, and could be changed or removed at any time.

The tracking issue is: `{}`
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
            *hidden_item.visibility_mut() = parse_quote! {
                pub(crate)
            };

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
    fn attrs(&self) -> &[syn::Attribute];

    fn push_attr(&mut self, attr: syn::Attribute);

    fn visibility(&self) -> &Visibility;

    fn visibility_mut(&mut self) -> &mut Visibility;

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

                fn visibility_mut(&mut self) -> &mut Visibility {
                    &mut self.vis
                }
            }
        )*
    };
}

impl_has_visibility!(
    syn::ItemType,
    syn::ItemEnum,
    syn::ItemStruct,
    syn::ItemFn,
    syn::ItemMod,
    syn::ItemTrait,
    syn::ItemConst,
    syn::ItemStatic,
);
