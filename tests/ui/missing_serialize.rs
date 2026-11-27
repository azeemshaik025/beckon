use beckon::beckon;

pub struct NewUser {
    pub name: String,
}

beckon!(
    Api,
    {
        { path: "/users", method: POST, req: NewUser, },
    }
);

fn main() {}
