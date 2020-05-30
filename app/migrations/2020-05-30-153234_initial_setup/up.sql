create table users
(
    id         varchar(255) not null primary key,
    hash       blob         not null,
    salt       varchar(255) not null,
    email      varchar(255) not null unique,
    created_at timestamp    not null default current_timestamp,
    updated_at timestamp    not null default current_timestamp on update current_timestamp,
    deleted    boolean      not null default 0
);

create table external_user_providers
(
    id         varchar(255)                         not null primary key,
    user_id    varchar(255)                         not null,
    external_id varchar(255) not null,
    email      varchar(255),
    provider   enum ('facebook', 'google', 'apple') not null,
    created_at timestamp                            not null default current_timestamp,
    updated_at timestamp                            not null default current_timestamp on update current_timestamp,
    deleted    boolean      not null default 0,
    constraint external_user_provider_fk_1
        foreign key (user_id) references users (id)
            on update cascade on delete cascade
);


create table auth_items
(
    name        varchar(64) not null primary key,
    type        smallint    not null,
    description text        null,
    created_at  timestamp   not null default current_timestamp,
    updated_at  timestamp   not null default current_timestamp on update current_timestamp
);

create table auth_assignments
(
    item_name  varchar(64)  not null,
    user_id    varchar(255) not null,
    created_at timestamp    not null default current_timestamp,
    primary key (item_name, user_id),
    constraint auth_assignment_fk_1
        foreign key (item_name) references auth_items (name)
            on update cascade on delete cascade
);

create table auth_item_children
(
    parent varchar(64) not null,
    child  varchar(64) not null,
    primary key (parent, child),
    constraint auth_item_child_fk_1
        foreign key (parent) references auth_items (name)
            on update cascade on delete cascade,
    constraint auth_item_child_fk_2
        foreign key (child) references auth_items (name)
            on update cascade on delete cascade
);

create table user_tokens
(
    id                varchar(255) charset utf8 not null primary key,
    token             mediumtext                not null,
    refresh_token     mediumtext                not null,
    user_id           varchar(255)              not null,
    refresh_expire_at timestamp                 not null default current_timestamp,
    created_at        timestamp                 not null default current_timestamp,
    updated_at        timestamp                 not null default current_timestamp on update current_timestamp,
    constraint user_tokens_fk_1
        foreign key (user_id) references users (id)
            on delete cascade,
    index user_id (user_id),
    fulltext index token (token),
    fulltext index refresh_token (refresh_token)
);