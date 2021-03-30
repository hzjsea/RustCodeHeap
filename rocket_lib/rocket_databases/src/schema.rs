table! {
    course (no) {
        no -> Varchar,
        name -> Varchar,
        t_no -> Varchar,
    }
}

table! {
    score (s_no, c_no) {
        s_no -> Varchar,
        c_no -> Varchar,
        degree -> Nullable<Decimal>,
    }
}

table! {
    student (no) {
        no -> Varchar,
        name -> Varchar,
        sex -> Varchar,
        birthday -> Nullable<Date>,
        class -> Nullable<Varchar>,
    }
}

table! {
    teacher (no) {
        no -> Varchar,
        name -> Varchar,
        sex -> Varchar,
        birthday -> Nullable<Date>,
        profession -> Varchar,
        department -> Varchar,
    }
}

joinable!(course -> teacher (t_no));
joinable!(score -> course (c_no));
joinable!(score -> student (s_no));

allow_tables_to_appear_in_same_query!(
    course,
    score,
    student,
    teacher,
);
