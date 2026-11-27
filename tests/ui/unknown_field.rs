use beckon::beckon;

beckon!(
    Api,
    {
        { path: "/users", methd: GET, },
    }
);

fn main() {}
