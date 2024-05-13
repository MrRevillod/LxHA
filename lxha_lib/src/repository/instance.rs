use mongodb::bson;
use std::sync::Arc;
use futures::TryStreamExt;

use std::vec;
use serde_json::Value;

use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId, Document},
};

use axum_responses::{extra::ToJson, AxumResult, HttpResponse};
use crate::app::DatabaseReference;
use crate::models::instance::*;

#[derive(Debug, Clone)]
pub struct InstanceRepository {
    collection: Collection<Instance>,
}

impl InstanceRepository {
    
    pub fn new(db: DatabaseReference) -> Self {
        InstanceRepository {
            collection: Arc::clone(&db).collection::<Instance>("instances"),
        }
    }

    pub async fn create(&self, instance: &Instance) -> AxumResult<()> {
        
        self.collection
            .insert_one(instance, None).await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)?
        ;

        Ok(())
    }

    pub async fn find_one_by_id(&self, id: &ObjectId) -> AxumResult<Option<Instance>> {

        let instance = self.collection
            .find_one(doc! { "_id": id }, None).await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)?
        ;

        Ok(instance)
    }

    pub async fn find_one(&self, filter: Document) -> AxumResult<Option<Instance>> {

        let instance = self.collection
            .find_one(filter, None).await
            .map_err(|e| HttpResponse::INTERNAL_SERVER_ERROR )?
        ;

        Ok(instance)
    }


    pub async fn find(&self) -> AxumResult<Vec<Value>> {

        let mut instances: Vec<Value> = vec![];
        let mut cursor = self.collection.find(None, None).await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)?
        ;

        while let Some(result) = cursor.try_next().await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)? {
            instances.push(result.to_json())
        }

        Ok(instances)
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
