// @generated automatically by Diesel CLI.

diesel::table! {
    blog (id) {
        id -> Int4,
        member_id -> Nullable<Int4>,
    }
}

diesel::table! {
    blog_post (id) {
        id -> Int4,
        blog_id -> Nullable<Int4>,
    }
}

diesel::table! {
    division (id) {
        id -> Int4,
        d_name -> Varchar,
    }
}

diesel::table! {
    event (id) {
        id -> Int4,
        e_name -> Varchar,
        e_desc -> Varchar,
        e_date -> Date,
        is_public -> Bool,
        profile_id -> Int4,
        program_id -> Int4,
    }
}

diesel::table! {
    event_image (event_id, image_id) {
        event_id -> Int4,
        image_id -> Int4,
    }
}

diesel::table! {
    forum (id) {
        id -> Int4,
        f_desc -> Varchar,
        forum_id -> Int4,
        member_id -> Int4,
    }
}

diesel::table! {
    forums (id) {
        id -> Int4,
        title -> Varchar,
        member_id -> Int4,
    }
}

diesel::table! {
    images (id) {
        id -> Int4,
        i_image -> Bytea,
        title -> Varchar,
    }
}

diesel::table! {
    improvement (id) {
        id -> Int4,
        body -> Varchar,
        likes -> Int4,
        dislikes -> Int4,
        event_id -> Nullable<Int4>,
        program_id -> Nullable<Int4>,
    }
}

diesel::table! {
    member (id) {
        id -> Int4,
        m_name -> Varchar,
        profile_id -> Nullable<Int4>,
        role -> Varchar,
        bio -> Varchar,
        joined -> Date,
        reported -> Int4,
        class -> Varchar,
        division_id -> Nullable<Int4>,
    }
}

diesel::table! {
    president (id) {
        id -> Int4,
        start -> Date,
        p_end -> Nullable<Date>,
    }
}

diesel::table! {
    program (id) {
        id -> Int4,
        p_name -> Varchar,
        p_desc -> Varchar,
        date_made -> Date,
        date_ended -> Nullable<Date>,
        is_public -> Bool,
        profile_id -> Nullable<Int4>,
        president_id -> Int4,
        status -> Int4,
    }
}

diesel::table! {
    program_image (program_id, image_id) {
        program_id -> Int4,
        image_id -> Int4,
    }
}

diesel::table! {
    reviews (id) {
        id -> Int4,
        title -> Varchar,
        body -> Varchar,
        event_id -> Nullable<Int4>,
        program_id -> Nullable<Int4>,
    }
}

diesel::table! {
    schedule (id) {
        id -> Int4,
        agenda -> Varchar,
        start -> Timestamp,
        s_length -> Interval,
        location -> Varchar,
        details -> Nullable<Varchar>,
        event_id -> Int4,
    }
}

diesel::table! {
    visit (id) {
        id -> Int4,
        time -> Timestamp,
        forums_id -> Int4,
    }
}

diesel::table! {
    worked_event (event_id, division_id) {
        event_id -> Int4,
        division_id -> Int4,
    }
}

diesel::table! {
    worked_program (program_id, division_id) {
        program_id -> Int4,
        division_id -> Int4,
    }
}

diesel::joinable!(blog -> member (member_id));
diesel::joinable!(blog_post -> blog (blog_id));
diesel::joinable!(event -> images (profile_id));
diesel::joinable!(event_image -> event (event_id));
diesel::joinable!(event_image -> images (image_id));
diesel::joinable!(forum -> forums (forum_id));
diesel::joinable!(forum -> member (member_id));
diesel::joinable!(forums -> member (member_id));
diesel::joinable!(improvement -> event (event_id));
diesel::joinable!(improvement -> program (program_id));
diesel::joinable!(member -> division (division_id));
diesel::joinable!(member -> images (profile_id));
diesel::joinable!(president -> member (id));
diesel::joinable!(program -> images (profile_id));
diesel::joinable!(program -> president (president_id));
diesel::joinable!(program_image -> images (image_id));
diesel::joinable!(program_image -> program (program_id));
diesel::joinable!(reviews -> event (event_id));
diesel::joinable!(reviews -> member (id));
diesel::joinable!(reviews -> program (program_id));
diesel::joinable!(schedule -> event (event_id));
diesel::joinable!(visit -> forums (forums_id));
diesel::joinable!(worked_event -> division (division_id));
diesel::joinable!(worked_event -> event (event_id));
diesel::joinable!(worked_program -> division (division_id));
diesel::joinable!(worked_program -> program (program_id));

diesel::allow_tables_to_appear_in_same_query!(
    blog,
    blog_post,
    division,
    event,
    event_image,
    forum,
    forums,
    images,
    improvement,
    member,
    president,
    program,
    program_image,
    reviews,
    schedule,
    visit,
    worked_event,
    worked_program,
);
