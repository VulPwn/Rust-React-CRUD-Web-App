table! {
    class_schedule (id) {
        id -> Int4,
        class_id -> Int4,
        class_date -> Date,
        class_end_date -> Date,
        md_recert_date -> Date,
        va_recert_date -> Date,
        dc_recert_date -> Date,
        instructor_id -> Int4,
        second_instructor_id -> Nullable<Int4>,
        cancelled -> Bool,
    }
}

table! {
    classes (id) {
        id -> Int4,
        class_title -> Text,
        class_language -> Text,
        md_approval_num -> Nullable<Text>,
        va_approval_num -> Nullable<Text>,
        dc_approval_num -> Nullable<Text>,
        md_recert_yrs -> Nullable<Int4>,
        va_recert_yrs -> Nullable<Int4>,
        dc_recert_yrs -> Nullable<Int4>,
        hours -> Int4,
    }
}

table! {
    companies (id) {
        id -> Int4,
        company_name -> Text,
        address -> Text,
        suite -> Nullable<Text>,
        city -> Text,
        state -> Text,
        zip_code -> Text,
        phone -> Text,
        phone_ext -> Nullable<Text>,
        email -> Text,
        poc_firstname -> Nullable<Text>,
        poc_lastname -> Nullable<Text>,
        cc_holdername -> Nullable<Text>,
        cc_num -> Nullable<Text>,
        cc_expdate -> Nullable<Text>,
        cc_cvv -> Nullable<Text>,
        cc_zipcode -> Nullable<Text>,
        notes -> Nullable<Text>,
    }
}

table! {
    instructors (id) {
        id -> Int4,
        acronym -> Text,
        first_name -> Text,
        last_name -> Text,
    }
}

table! {
    studentclass (id) {
        id -> Int4,
        class_id -> Int4,
        student_id -> Int4,
        certification_num -> Nullable<Text>,
        test_score -> Nullable<Int4>,
        class_date -> Date,
        class_end_date -> Date,
        md_recert_date -> Date,
        va_recert_date -> Date,
        dc_recert_date -> Date,
    }
}

table! {
    students (id) {
        id -> Int4,
        social -> Text,
        first_name -> Text,
        last_name -> Text,
        address -> Text,
        suite -> Nullable<Text>,
        city -> Text,
        state -> Text,
        zip_code -> Text,
        phone -> Text,
        dob -> Date,
        company_id -> Nullable<Int4>,
        email -> Nullable<Text>,
        photo -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        pass -> Text,
        salt -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    class_schedule,
    classes,
    companies,
    instructors,
    studentclass,
    students,
    users,
);
