use syn::Visibility;

pub trait ItemLike {
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
