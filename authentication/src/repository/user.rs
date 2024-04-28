
use std::sync::Arc;
use futures::TryStreamExt;
use mongodb::bson;

use serde_json::Value;

use std::vec;

use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId, Document},
};

use axum_responses::{extra::ToJson, AxumResult, HttpResponse};

use crate::{
    models::user::*,
    config::types::DatabaseReference,
};

#[derive(Debug, Clone)]
pub struct UserRespository {
    db: DatabaseReference,
    collection: Collection<User>,
}

impl UserRespository {
    
    pub fn new(db: DatabaseReference) -> Self {
        UserRespository {
            db: Arc::clone(&db),
            collection: Arc::clone(&db).collection::<User>("users"),
        }
    }

    pub async fn create(&self, user: &User) -> AxumResult<()> {
        
        self.collection
            .insert_one(user, None).await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)?
        ;

        Ok(())
    }

    pub async fn find_one_by_id(&self, id: &ObjectId) -> AxumResult<Option<User>> {

        let user = self.collection
            .find_one(doc! { "_id": id }, None).await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)?
        ;

        Ok(user)
    }

    pub async fn find_one(&self, filter: Document) -> AxumResult<Option<User>> {

        let user = self.collection
            .find_one(filter, None).await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)?
        ;

        Ok(user)
    }

    pub async fn find_one_with_instances(&self, id: &ObjectId) -> AxumResult<Option<ProfileWithInstance>> {

        let pipeline = vec![
            doc! {
                "$match": {
                    "_id": id,
                }
            },
            doc! {
                "$lookup": {
                    "from": "instances",
                    "localField": "instances",
                    "foreignField": "_id",
                    "as": "instances",
                }
            },
            doc! {
                "$unwind": {
                    "path": "$instances",
                }
            },
            doc! {
                "$replaceRoot": {
                    "newRoot": {
                        "$mergeObjects": ["$instances", "$$ROOT"],
                    }
                }
            },
            doc! {
                "$project": {
                    "_id": 1,
                    "username": 1,
                    "email": 1,
                    "password": 1,
                    "validated": 1,
                    "role": 1,
                    "instances": 1,
                }
            },
        ];

        let mut cursor = self.collection.aggregate(pipeline, None).await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)?
        ;

        let user_with_instances = cursor.try_next().await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)?
            .map(|doc| bson::from_document::<ProfileWithInstance>(doc).unwrap())
        ;

        Ok(user_with_instances)
    }

    pub async fn find(&self) -> AxumResult<Vec<Value>> {

        let mut profiles: Vec<Value> = vec![];
        let mut cursor = self.collection.find(None, None).await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)?
        ;

        while let Some(result) = cursor.try_next().await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)? {

            profiles.push(result.into_profile().to_json())
        }

        Ok(profiles)
    }

    pub async fn update(&self, id: &ObjectId, update: Document) -> AxumResult<()> {

        self.collection
            .update_one(doc! { "_id": id }, update, None).await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)?
        ;

        Ok(())
    }

    pub async fn delete(&self, id: &ObjectId) -> AxumResult<()> {

        self.collection
            .delete_one(doc! { "_id": id }, None).await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)?
        ;

        Ok(())
    }
}


