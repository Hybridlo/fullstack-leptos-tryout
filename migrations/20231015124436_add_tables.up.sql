-- Add up migration script here
CREATE TABLE category (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    name text NOT NULL
);

CREATE TABLE item (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    name text NOT NULL,
    category_id uuid NOT NULL,
    CONSTRAINT fk_category
        FOREIGN KEY(category_id)
            REFERENCES category(id)
);

CREATE TABLE tag (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    name text NOT NULL
);

CREATE TABLE item_tag (
    item_id uuid,
    tag_id uuid,
    PRIMARY KEY (item_id, tag_id),
    
    CONSTRAINT fk_item
        FOREIGN KEY(item_id)
            REFERENCES item(id)
                ON DELETE CASCADE,
    
    CONSTRAINT fk_tag
        FOREIGN KEY(tag_id)
            REFERENCES tag(id)
                ON DELETE CASCADE
);

CREATE TABLE item_objects (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    item_code text,
    item_id uuid NOT NULL,
    CONSTRAINT fk_item
        FOREIGN KEY(item_id)
            REFERENCES item(id)
                ON DELETE CASCADE
);