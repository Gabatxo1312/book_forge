use entity::{book, user};
use sea_orm::ActiveModelTrait;
use sea_orm::{ActiveValue::Set, DatabaseConnection, DbErr};

#[allow(dead_code)]
pub struct TestData {
    pub users: Vec<user::Model>,
    pub books: Vec<book::Model>,
}

impl TestData {
    pub async fn seed_fake_books_data(db: &DatabaseConnection) -> Result<TestData, DbErr> {
        let user1 = user::ActiveModel {
            name: Set("Karl Marx".to_string()),
            ..Default::default()
        };

        let user2 = user::ActiveModel {
            name: Set("Leon Tolstoi".to_string()),
            ..Default::default()
        };

        let user3 = user::ActiveModel {
            name: Set("Koprotkine".to_string()),
            ..Default::default()
        };

        let saved_user1 = user1.insert(db).await?;
        let saved_user2 = user2.insert(db).await?;
        let saved_book3 = user3.insert(db).await?;

        let book1 = book::ActiveModel {
            title: Set("Poulets grillés".to_string()),
            owner_id: Set(saved_user1.id),
            description: Set(Some("Le 36 quai des Orfèvres s'offre un nouveau patron. Le but de la manœuvre  : faire briller les statistiques en placardisant tous ceux qu'on ne peut pas virer et qui encombrent les services. Nommée à la tête de ce ramassis d'alcoolos, de porte-poisse, d'homos, d'écrivains et autres crétins, Anne Capestan, étoile déchue de la Judiciaire, a bien compris que sa mission était de se taire. Mais voilà, elle déteste obéir et puis... il ne faut jamais vendre la peau des poulets grillés avant de les avoir plumés !".to_string())),
            cover_url: Set(Some("teststestsesdf".to_string())),
            open_library_link: Set(Some("teststestsesdf".to_string())),
            current_holder_id: Set(None),
            authors: Set("Sophie Hénaff".to_string()),
            ..Default::default()
        };

        let saved_book1 = book1.insert(db).await?;

        Ok(Self {
            books: vec![saved_book1],
            users: vec![saved_user1, saved_user2, saved_book3],
        })
    }
}
