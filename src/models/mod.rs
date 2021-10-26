pub mod schema;

pub mod user;
pub mod user_detail;
pub mod todo;

#[macro_export]
macro_rules! new_data_struct {
    (
        $n:ident,
        $nn:ident,
        $table:literal,
        $id_ty:ty,
        items=>[
            $($f:ident : $t:ty | $nt:ty),*
            ]
        ) => {
        #[derive(Queryable,Identifiable)]
        #[table_name=$table]
        pub struct $n {
            pub id : $id_ty,
            $( pub $f : $t ),*
        }
        #[derive(Insertable)]
        #[table_name=$table]
        pub struct $nn {
            $( pub $f:$nt ),*
        }
    };
    (
        $n:ident,
        $nn:ident,
        $nnl:lifetime, 
        $table:literal,
        $id_ty:ty,
        items=>[
            $($f:ident : $t:ty | $nt:ty),*
            ]
        ) => {
        #[derive(Queryable,Identifiable)]
        #[table_name=$table]
        pub struct $n {
            pub id : $id_ty,
            $( pub $f : $t ),*
        }
        #[derive(Insertable)]
        #[table_name=$table]
        pub struct $nn < $nnl > {
            $( pub $f:$nt ),*
        }
    };
}