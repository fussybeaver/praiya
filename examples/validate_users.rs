//! This example will fetch all users and check their notification rules

use futures_util::TryStreamExt;
use praiya::Praiya;

use praiya::api::users::ListUsersParamsBuilder;
use praiya::models::{ContactMethod, ContactMethodTypeEnum, NotificationRule, User, UserRoleEnum};
use praiya::ParamsBuilder;

// Replace with appropriate values...
const PAGERDUTY_TOKEN: &str = "";

#[tokio::main]
async fn main() -> Result<(), praiya::errors::Error> {
    let pagerduty = Praiya::new(PAGERDUTY_TOKEN);

    let params = {
        let mut option = ListUsersParamsBuilder::new();
        let include = vec!["notification_rules"];

        option.include(include);
        option.build()
    };

    let users: Vec<User> = pagerduty
        .users()
        .list_users(params)
        .try_collect()
        .await
        .expect("failed to fetch users");

    let default_contact_types = vec![
        ContactMethodTypeEnum::EMAIL_CONTACT_METHOD,
        ContactMethodTypeEnum::SMS_CONTACT_METHOD,
        ContactMethodTypeEnum::PHONE_CONTACT_METHOD,
    ];

    for user in &users {
        match user {
            User {
                role: Some(role),
                notification_rules: Some(rules),
                summary: Some(summary),
                ..
            } if !vec![
                UserRoleEnum::READ_ONLY_USER,
                UserRoleEnum::READ_ONLY_LIMITED_USER,
            ]
            .iter()
            .any(|r| r == role)
                && default_contact_types.iter().all(|c| {
                    rules.iter().any(|r| match r {
                        NotificationRule {
                            contact_method: Some(ContactMethod { _type, .. }),
                            ..
                        } if c == _type => true,
                        _ => false,
                    })
                }) =>
            {
                println!("User is valid: {}", &summary);
            }
            User {
                summary: Some(summary),
                role: Some(role),
                notification_rules: Some(notification_rules),
                ..
            } => println!(
                "User is not a responder, or missing notification rules: {:?}, {:?}, {:?}",
                summary,
                role,
                notification_rules
                    .iter()
                    .map(|n| n.contact_method.as_ref().unwrap()._type.as_ref())
                    .collect::<Vec<&str>>()
            ),
            _ => println!("Failed to parse user {:?}", user),
        }
    }

    Ok(())
}
