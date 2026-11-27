use beckon::beckon;

pub struct UserPath {
    pub user_id: u32,
}

beckon!(
    Api,
    {
        { path: "/users/{id}", method: GET, path_params: UserPath, },
    }
);

fn main() {}
