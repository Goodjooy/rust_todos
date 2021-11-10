
use diesel::QueryDsl;




pub struct SelectBuider<T:QueryDsl> {
    dls:T
}