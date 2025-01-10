use syn::Visibility;

pub trait Stability {
    #[allow(unused)]
    fn attrs(&self) -> &[syn::Attribute];

    fn push_attr(&mut self, attr: syn::Attribute);
}

pub trait ItemLike: Stability {
    fn visibility(&self) -> &Visibility;

    fn set_visibility(&mut self, visibility: Visibility);

    fn is_public(&self) -> bool {
        matches!(self.visibility(), Visibility::Public(_))
    }

    fn allowed_lints(&self) -> Vec<syn::Ident>;
}

macro_rules! impl_has_visibility {
($ty:ty[$($allows:ident),*]) => {
    impl Stability for $ty {
        fn attrs(&self) -> &[syn::Attribute] {
            &self.attrs
        }

        fn push_attr(&mut self, attr: syn::Attribute) {
            self.attrs.push(attr);
        }
    }

    impl ItemLike for $ty {
        fn visibility(&self) -> &Visibility {
            &self.vis
        }

        fn set_visibility(&mut self, visibility: Visibility) {
            self.vis = visibility;
        }

        fn allowed_lints(&self) -> Vec<syn::Ident> {
            vec![
                $(syn::Ident::new(stringify!($allows), proc_macro2::Span::call_site()),)*
            ]
        }
    }
};
($ty:ty) => {
    $crate::item_like::impl_has_visibility!($ty [dead_code]);
};
($($ty:ty $([$($allows:ident),*])?),+ $(,)?) => {
    $(
        $crate::item_like::impl_has_visibility!($ty $([$($allows),*])?);
    )*
};
}

pub(crate) use impl_has_visibility;

impl_has_visibility!(
    syn::ItemType,
    syn::ItemEnum,
    syn::ItemFn,
    syn::ItemMod,
    syn::ItemTrait,
    syn::ItemConst,
    syn::ItemStatic,
    syn::ItemUse[unused_imports],
);

impl Stability for syn::ItemStruct {
    fn attrs(&self) -> &[syn::Attribute] {
        &self.attrs
    }

    fn push_attr(&mut self, attr: syn::Attribute) {
        self.attrs.push(attr);
    }
}

impl ItemLike for syn::ItemStruct {
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

    fn allowed_lints(&self) -> Vec<syn::Ident> {
        vec![syn::Ident::new("dead_code", proc_macro2::Span::call_site())]
    }
}

impl Stability for syn::ItemImpl {
    fn attrs(&self) -> &[syn::Attribute] {
        &self.attrs
    }

    fn push_attr(&mut self, attr: syn::Attribute) {
        self.attrs.push(attr);
    }
}
