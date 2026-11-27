use beckon::beckon;

beckon!(
    Api,
    {
        { path: "/users/{id}", method: GET, },
    }
);

fn main() {}
