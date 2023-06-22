#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod framework {
    // use ink::storage::Mapping;
    use ink::prelude::{
        string::String,
        vec::Vec
    };
    use ink::storage::Lazy;
    use tasknet::TaskNetRef;
    use ink::TypeInfo;

    #[derive(Debug, PartialEq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum FrameworkError {
        UsernameTaken,
        UserAlreadyResponded,
        TaskRewardTooLow
    }

    #[derive(scale::Decode, scale::Encode, Debug, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct User {
        address: AccountId,
        username: String,
        experience: Vec<String>,
        skills: Vec<String>,
        // history: Vec<String> // potentially a hash of a completed tasknet
    }

    impl User {
        pub fn new(
            address: AccountId,
            username: String,
            experience: Vec<String>,
            skills: Vec<String>
        ) -> Self {
            Self {
                address,
                username,
                experience,
                skills
            }
        }
    }

    #[ink(storage)]
    pub struct Framework {
        users: Vec<User>,
        next_user_id: i64,
        task_net: TaskNetRef,
        // ml_net: Mapping<i64, ML>,
    }

    impl Framework {
        #[ink(constructor)]
        pub fn new(tasknet_hash: Hash) -> Self {
            let tasknet = TaskNetRef::new()
                .code_hash(tasknet_hash)
                .endowment(0)
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate();;
            Self {
                users: Vec::new(),
                next_user_id: 0,
                task_net: tasknet
            }
        }

        #[ink(message)]
        pub fn create_user(
            &mut self,
            username: String,
            experience: Vec<String>,
            skills: Vec<String>
        ) {
            let caller: AccountId = Self::env().caller();

            // Create users if address isn't linked to an account
            if !self.users.iter().any(|user| user.username == username) {
                let mut user = User::new(
                    caller,
                    username,
                    Vec::<String>::with_capacity(10),
                    Vec::<String>::with_capacity(10),
                );

                // Push experiences to users if specified
                for i in 0..experience.len().min(user.experience.capacity()) {
                    user.experience.push(experience.get(i)
                                    .cloned()
                                    .unwrap_or_else(|| String::new()));
                }

                // Push skills to users if specified
                for i in 0..skills.len().min(user.skills.capacity()) {
                    user.skills.push(skills.get(i)
                                    .cloned()
                                    .unwrap_or_else(|| String::new()));
                }

                // Add users to contract
                self.users.push(user);
            } else {
                // return Err(FrameworkError::UserAlreadyExists);
            }
        }

        #[ink(message)]
        pub fn get_user(&self, username: String) -> Option<User> {
            let mut user: Option<User> = None;

            for _user in self.users.iter() {
                if _user.username == username {
                    user = Some(_user.clone());
                }
            }

            user
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        pub fn framework_test() {
            let mut contract: Framework = Framework::new();
            let caller: AccountId = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
            let username = String::from("Alice");
            let experience = vec![String::from("Software Developer"), String::from("Data Analyst")];
            let skills = vec![
                String::from("Rust"),
                String::from("Python"),
                String::from("SQL")
            ];

            contract.create_user(
                username.clone(),
                experience.clone(),
                skills.clone()
            );

            let user = contract.get_user(username).unwrap();

            println!("Username: {}", user.username);
            assert_eq!(caller, user.address);
        }
    }
}
            // contract.create_task(
            //     String::from("Lens Network Flagged Comment: Your mom's a wanker"),
            //     String::from("Was this post harmful? (y/n)"),
            //     1,
            // );
            //
            // let task = contract.get_task(0).unwrap();

            // println!("tasknet Title: {}", framework.users);
            // println!("Description: {}");
            // println!("Reward: {} AZERO", framework.users.get());
