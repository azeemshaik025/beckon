use beckon::beckon;

pub struct User {
    pub id: u32,
}

beckon!(
    Api,
    {
        { path: "/users", method: GET, res: User, },
    }
);

fn main() {}
