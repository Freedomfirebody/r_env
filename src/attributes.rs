use bae2::FromAttributes;

#[derive(FromAttributes)]
pub struct EnvAttr {
    pub name: syn::Lit,
    pub default: Option<syn::Lit>,
    pub exec: Option<syn::Expr>
}