use beckon::beckon;

beckon!(
    Api,
    {
        { path: "/users", method: FETCH, },
    }
);

fn main() {}
