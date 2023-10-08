// @generated automatically by Diesel CLI.

diesel::table! {
    airport (airport_code) {
        #[max_length = 32]
        airport_code -> Varchar,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 32]
        continent -> Varchar,
        #[sql_name = "type"]
        #[max_length = 32]
        type_ -> Varchar,
        lat -> Float4,
        lon -> Float4,
        site -> Nullable<Text>,
        wiki -> Nullable<Text>,
        #[max_length = 50]
        iso_region -> Varchar,
        #[max_length = 32]
        gps_code -> Nullable<Varchar>,
    }
}

diesel::table! {
    client (client_id) {
        client_id -> Int4,
        #[max_length = 50]
        login -> Varchar,
        password -> Text,
        profile_id -> Int4,
    }
}

diesel::table! {
    country (iso_country) {
        #[max_length = 50]
        iso_country -> Varchar,
        #[max_length = 50]
        continent -> Varchar,
        wikipedia_link -> Text,
        #[max_length = 100]
        name -> Varchar,
    }
}

diesel::table! {
    deny (deny_id, request_id) {
        deny_id -> Int4,
        request_id -> Int4,
        reason -> Text,
    }
}

diesel::table! {
    employee (employee_id) {
        employee_id -> Int4,
        #[max_length = 50]
        login -> Varchar,
        post_id -> Int4,
        password -> Text,
        profile_id -> Int4,
        salary -> Numeric,
    }
}

diesel::table! {
    feature (feature_id) {
        feature_id -> Int4,
        #[max_length = 64]
        name -> Varchar,
        icon -> Text,
        desctiption -> Text,
    }
}

diesel::table! {
    flight (flight_id) {
        flight_id -> Int4,
        plane_id -> Int4,
        route_id -> Int4,
    }
}

diesel::table! {
    plane (plane_id) {
        plane_id -> Int4,
        #[max_length = 64]
        model -> Varchar,
        description -> Text,
        seats_count -> Int4,
        luggage_max -> Int4,
    }
}

diesel::table! {
    plane_feature (plane_feature_id) {
        plane_feature_id -> Int4,
        plane_id -> Int4,
        feature_id -> Int4,
    }
}

diesel::table! {
    post (post_id) {
        post_id -> Int4,
        #[max_length = 64]
        name -> Varchar,
    }
}

diesel::table! {
    profile (profile_id) {
        profile_id -> Int4,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 10]
        passport -> Varchar,
        #[max_length = 64]
        surname -> Varchar,
        #[max_length = 64]
        firstname -> Varchar,
        #[max_length = 64]
        middlename -> Varchar,
        isdeleted -> Bool,
    }
}

diesel::table! {
    receipt (receipt_id) {
        receipt_id -> Int4,
        created_at -> Timestamp,
        #[max_length = 32]
        payment_method -> Varchar,
        paymentsumm -> Int4,
        ticket_id -> Int4,
    }
}

diesel::table! {
    region (iso_region) {
        #[max_length = 50]
        iso_region -> Varchar,
        #[max_length = 50]
        continent -> Varchar,
        #[max_length = 50]
        iso_country -> Varchar,
        #[max_length = 100]
        name -> Varchar,
        wikipedia_link -> Text,
    }
}

diesel::table! {
    request (request_id) {
        request_id -> Int4,
        created_at -> Timestamp,
        isdeny -> Bool,
        client_id -> Int4,
        route_id -> Int4,
    }
}

diesel::table! {
    route (route_id) {
        route_id -> Int4,
        #[max_length = 32]
        code_from -> Varchar,
        #[max_length = 32]
        code_to -> Varchar,
    }
}

diesel::table! {
    ticket (ticket_id) {
        ticket_id -> Int4,
        buy_at -> Timestamp,
        isscanned -> Bool,
        scanned_at -> Nullable<Timestamp>,
        paysumm -> Int4,
        request_id -> Int4,
        employee_id -> Nullable<Int4>,
        flight_id -> Int4,
        place_number -> Int4,
    }
}

diesel::joinable!(airport -> region (iso_region));
diesel::joinable!(client -> profile (profile_id));
diesel::joinable!(deny -> request (request_id));
diesel::joinable!(employee -> post (post_id));
diesel::joinable!(employee -> profile (profile_id));
diesel::joinable!(flight -> plane (plane_id));
diesel::joinable!(flight -> route (route_id));
diesel::joinable!(plane_feature -> feature (feature_id));
diesel::joinable!(plane_feature -> plane (plane_id));
diesel::joinable!(receipt -> ticket (ticket_id));
diesel::joinable!(region -> country (iso_country));
diesel::joinable!(request -> client (client_id));
diesel::joinable!(request -> route (route_id));
diesel::joinable!(ticket -> employee (employee_id));
diesel::joinable!(ticket -> flight (flight_id));
diesel::joinable!(ticket -> request (request_id));

diesel::allow_tables_to_appear_in_same_query!(
    airport,
    client,
    country,
    deny,
    employee,
    feature,
    flight,
    plane,
    plane_feature,
    post,
    profile,
    receipt,
    region,
    request,
    route,
    ticket,
);
