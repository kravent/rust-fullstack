use shaku::module;

use crate::user::UserRepositoryImpl;

module! {
    pub AppModule {
        components = [UserRepositoryImpl],
        providers = []
    }
}
