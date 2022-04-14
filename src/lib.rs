use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Index};

#[proc_macro_derive(Vs)]
pub fn derive_vsmgmt(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let version_create = gen_version_create(&input.data);
    let version_create_by_branch = gen_version_create_by_branch(&input.data);
    let version_exists = gen_version_exists(&input.data);
    let version_exists_on_branch = gen_version_exists_on_branch(&input.data);
    let version_pop = gen_version_pop(&input.data);
    let version_pop_by_branch = gen_version_pop_by_branch(&input.data);
    let version_rebase = gen_version_rebase(&input.data);
    let version_rebase_by_branch = gen_version_rebase_by_branch(&input.data);

    let branch_create = gen_branch_create(&input.data);
    let branch_create_by_base_branch = gen_branch_create_by_base_branch(&input.data);
    let branch_create_by_base_branch_version =
        gen_branch_create_by_base_branch_version(&input.data);

    let branch_create_without_new_version =
        gen_branch_create_without_new_version(&input.data);
    let branch_create_by_base_branch_without_new_version =
        gen_branch_create_by_base_branch_without_new_version(&input.data);
    let branch_create_by_base_branch_version_without_new_version =
        gen_branch_create_by_base_branch_version_without_new_version(&input.data);

    let branch_exists = gen_branch_exists(&input.data);
    let branch_has_versions = gen_branch_has_versions(&input.data);
    let branch_remove = gen_branch_remove(&input.data);
    let branch_truncate = gen_branch_truncate(&input.data);
    let branch_truncate_to = gen_branch_truncate_to(&input.data);
    let branch_pop_version = gen_branch_pop_version(&input.data);
    let branch_merge_to = gen_branch_merge_to(&input.data);
    let branch_merge_to_force = gen_branch_merge_to_force(&input.data);
    let branch_set_default = gen_branch_set_default(&input.data);
    let prune = gen_prune(&input.data);

    let expanded = quote! {
        use ruc::*;
        impl #impl_generics vsdb::VsMgmt for #name #ty_generics #where_clause {
            fn version_create(&self, version_name: vsdb::VersionName) -> Result<()> {
                #version_create
                Ok(())
            }

            fn version_create_by_branch(
                &self,
                version_name: vsdb::VersionName,
                branch_name: vsdb::BranchName,
                ) -> Result<()> {
                #version_create_by_branch
                Ok(())
            }

            fn version_exists(&self, version_name: vsdb::VersionName) -> bool {
                #version_exists
            }

            fn version_exists_on_branch(
                &self,
                version_name: vsdb::VersionName,
                branch_name: vsdb::BranchName,
                ) -> bool {
                #version_exists_on_branch
            }

            fn version_pop(&self) -> Result<()> {
                #version_pop
                Ok(())
            }

            fn version_pop_by_branch(&self, branch_name: vsdb::BranchName) -> Result<()> {
                #version_pop_by_branch
                Ok(())
            }

            unsafe fn version_rebase(&self, base_version: vsdb::VersionName) -> Result<()> {
                #version_rebase
                Ok(())
            }

            unsafe fn version_rebase_by_branch(&self, base_version: vsdb::VersionName, branch_name: vsdb::BranchName) -> Result<()> {
                #version_rebase_by_branch
                Ok(())
            }

            fn branch_create(&self, branch_name: vsdb::BranchName, version_name: vsdb::VersionName) -> Result<()> {
                #branch_create
                Ok(())
            }

            fn branch_create_by_base_branch(
                &self,
                branch_name: vsdb::BranchName,
                version_name: vsdb::VersionName,
                base_branch_name: vsdb::ParentBranchName,
            ) -> Result<()> {
                #branch_create_by_base_branch
                Ok(())
            }

            fn branch_create_by_base_branch_version(
                &self,
                branch_name: vsdb::BranchName,
                version_name: vsdb::VersionName,
                base_branch_name: vsdb::ParentBranchName,
                base_version_name: vsdb::VersionName,
            ) -> Result<()> {
                #branch_create_by_base_branch_version
                Ok(())
            }

            unsafe fn branch_create_without_new_version(&self, branch_name: vsdb::BranchName) -> Result<()> {
                #branch_create_without_new_version
                Ok(())
            }

            unsafe fn branch_create_by_base_branch_without_new_version(
                &self,
                branch_name: vsdb::BranchName,
                base_branch_name: vsdb::ParentBranchName,
            ) -> Result<()> {
                #branch_create_by_base_branch_without_new_version
                Ok(())
            }

            unsafe fn branch_create_by_base_branch_version_without_new_version(
                &self,
                branch_name: vsdb::BranchName,
                base_branch_name: vsdb::ParentBranchName,
                base_version_name: vsdb::VersionName,
            ) -> Result<()> {
                #branch_create_by_base_branch_version_without_new_version
                Ok(())
            }

            fn branch_exists(&self, branch_name: vsdb::BranchName) -> bool {
                #branch_exists
            }

            fn branch_has_versions(&self, branch_name: vsdb::BranchName) -> bool {
                #branch_has_versions
            }

            fn branch_remove(&self, branch_name: vsdb::BranchName) -> Result<()> {
                #branch_remove
                Ok(())
            }

            fn branch_truncate(&self, branch_name: vsdb::BranchName) -> Result<()> {
                #branch_truncate
                Ok(())
            }

            fn branch_truncate_to(
                &self,
                branch_name: vsdb::BranchName,
                last_version_name: vsdb::VersionName,
            ) -> Result<()> {
                #branch_truncate_to
                Ok(())
            }

            fn branch_pop_version(&self, branch_name: vsdb::BranchName) -> Result<()> {
                #branch_pop_version
                Ok(())
            }

            fn branch_merge_to(&self, branch_name: vsdb::BranchName, target_branch_name: vsdb::BranchName) -> Result<()> {
                #branch_merge_to
                Ok(())
            }

            unsafe fn branch_merge_to_force(&self, branch_name: vsdb::BranchName, target_branch_name: vsdb::BranchName) -> Result<()> {
                #branch_merge_to_force
                Ok(())
            }

            fn branch_set_default(&mut self, branch_name: vsdb::BranchName) -> Result<()> {
                #branch_set_default
                Ok(())
            }

            fn prune(&self, reserved_ver_num: Option<usize>) -> Result<()> {
                #prune
                Ok(())
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

fn gen_version_create(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let id = &f.ident;
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::version_create(&self.#id, version_name).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let id = Index::from(i);
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::version_create(&self.#id, version_name).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

fn gen_version_create_by_branch(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                        let id = &f.ident;
                        quote_spanned! {f.span()=>
                            vsdb::VsMgmt::version_create_by_branch(&self.#id, version_name, branch_name).c(d!())?;
                        }
                    });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let id = Index::from(i);
                        quote_spanned! {f.span()=>
                            vsdb::VsMgmt::version_create_by_branch(&self.#id, version_name, branch_name).c(d!())?;
                        }
                    });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

fn gen_version_exists(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let id = &f.ident;
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::version_exists(&self.#id, version_name) &&
                    }
                });
                quote! {
                    #(#recurse)* true
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let id = Index::from(i);
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::version_exists(&self.#id, version_name) &&
                    }
                });
                quote! {
                    #(#recurse)* true
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

fn gen_version_exists_on_branch(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                        let id = &f.ident;
                        quote_spanned! {f.span()=>
                            vsdb::VsMgmt::version_exists_on_branch(&self.#id, version_name, branch_name) &&
                        }
                    });
                quote! {
                    #(#recurse)* true
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let id = Index::from(i);
                        quote_spanned! {f.span()=>
                            vsdb::VsMgmt::version_exists_on_branch(&self.#id, version_name, branch_name) &&
                        }
                    });
                quote! {
                    #(#recurse)* true
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

fn gen_version_pop(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let id = &f.ident;
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::version_pop(&self.#id).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let id = Index::from(i);
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::version_pop(&self.#id).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

fn gen_version_pop_by_branch(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let id = &f.ident;
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::version_pop_by_branch(&self.#id, branch_name).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let id = Index::from(i);
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::version_pop_by_branch(&self.#id, branch_name).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

fn gen_version_rebase(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let id = &f.ident;
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::version_rebase(&self.#id, base_version).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let id = Index::from(i);
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::version_rebase(&self.#id, base_version).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

fn gen_version_rebase_by_branch(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let id = &f.ident;
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::version_rebase_by_branch(&self.#id, base_version, branch_name).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let id = Index::from(i);
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::version_rebase_by_branch(&self.#id, base_version, branch_name).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

fn gen_branch_create(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let id = &f.ident;
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::branch_create(&self.#id, branch_name, version_name).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let id = Index::from(i);
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::branch_create(&self.#id, branch_name, version_name).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

fn gen_branch_create_by_base_branch(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                        let id = &f.ident;
                        quote_spanned! {f.span()=>
                            vsdb::VsMgmt::branch_create_by_base_branch(&self.#id, branch_name, version_name, base_branch_name).c(d!())?;
                        }
                    });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let id = Index::from(i);
                        quote_spanned! {f.span()=>
                            vsdb::VsMgmt::branch_create_by_base_branch(&self.#id, branch_name, version_name, base_branch_name).c(d!())?;
                        }
                    });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

fn gen_branch_create_by_base_branch_version(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                        let id = &f.ident;
                        quote_spanned! {f.span()=>
                            vsdb::VsMgmt::branch_create_by_base_branch_version(&self.#id, branch_name, version_name, base_branch_name, base_version_name).c(d!())?;
                        }
                    });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let id = Index::from(i);
                        quote_spanned! {f.span()=>
                            vsdb::VsMgmt::branch_create_by_base_branch_version(&self.#id, branch_name, version_name, base_branch_name, base_version_name).c(d!())?;
                        }
                    });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

fn gen_branch_create_without_new_version(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let id = &f.ident;
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::branch_create_without_new_version(&self.#id, branch_name).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let id = Index::from(i);
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::branch_create_without_new_version(&self.#id, branch_name).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

fn gen_branch_create_by_base_branch_without_new_version(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                        let id = &f.ident;
                        quote_spanned! {f.span()=>
                            vsdb::VsMgmt::branch_create_by_base_branch_without_new_version(&self.#id, branch_name, base_branch_name).c(d!())?;
                        }
                    });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let id = Index::from(i);
                        quote_spanned! {f.span()=>
                            vsdb::VsMgmt::branch_create_by_base_branch_without_new_version(&self.#id, branch_name, base_branch_name).c(d!())?;
                        }
                    });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

fn gen_branch_create_by_base_branch_version_without_new_version(
    data: &Data,
) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                        let id = &f.ident;
                        quote_spanned! {f.span()=>
                            vsdb::VsMgmt::branch_create_by_base_branch_version_without_new_version(&self.#id, branch_name, base_branch_name, base_version_name).c(d!())?;
                        }
                    });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let id = Index::from(i);
                        quote_spanned! {f.span()=>
                            vsdb::VsMgmt::branch_create_by_base_branch_version_without_new_version(&self.#id, branch_name, base_branch_name, base_version_name).c(d!())?;
                        }
                    });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

fn gen_branch_exists(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let id = &f.ident;
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::branch_exists(&self.#id, branch_name) &&
                    }
                });
                quote! {
                    #(#recurse)* true
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let id = Index::from(i);
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::branch_exists(&self.#id, branch_name) &&
                    }
                });
                quote! {
                    #(#recurse)* true
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}
fn gen_branch_has_versions(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let id = &f.ident;
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::branch_has_versions(&self.#id, branch_name) &&
                    }
                });
                quote! {
                    #(#recurse)* true
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let id = Index::from(i);
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::branch_has_versions(&self.#id, branch_name) &&
                    }
                });
                quote! {
                    #(#recurse)* true
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

fn gen_branch_remove(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let id = &f.ident;
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::branch_remove(&self.#id, branch_name).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let id = Index::from(i);
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::branch_remove(&self.#id, branch_name).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

fn gen_branch_truncate(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let id = &f.ident;
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::branch_truncate(&self.#id, branch_name).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let id = Index::from(i);
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::branch_truncate(&self.#id, branch_name).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

fn gen_branch_truncate_to(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                        let id = &f.ident;
                        quote_spanned! {f.span()=>
                            vsdb::VsMgmt::branch_truncate_to(&self.#id, branch_name, last_version_name).c(d!())?;
                        }
                    });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let id = Index::from(i);
                        quote_spanned! {f.span()=>
                            vsdb::VsMgmt::branch_truncate_to(&self.#id, branch_name, last_version_name).c(d!())?;
                        }
                    });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

fn gen_branch_pop_version(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let id = &f.ident;
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::branch_pop_version(&self.#id, branch_name).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let id = Index::from(i);
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::branch_pop_version(&self.#id, branch_name).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

fn gen_branch_merge_to(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let id = &f.ident;
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::branch_merge_to(&self.#id, branch_name, target_branch_name).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let id = Index::from(i);
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::branch_merge_to(&self.#id, branch_name, target_branch_name).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

fn gen_branch_merge_to_force(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let id = &f.ident;
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::branch_merge_to_force(&self.#id, branch_name, target_branch_name).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let id = Index::from(i);
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::branch_merge_to_force(&self.#id, branch_name, target_branch_name).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

fn gen_branch_set_default(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let id = &f.ident;
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::branch_set_default(&mut self.#id, branch_name).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let id = Index::from(i);
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::branch_set_default(&mut self.#id, branch_name).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}

fn gen_prune(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let id = &f.ident;
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::prune(&self.#id, reserved_ver_num).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let id = Index::from(i);
                    quote_spanned! {f.span()=>
                        vsdb::VsMgmt::prune(&self.#id, reserved_ver_num).c(d!())?;
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => todo!(),
    }
}
