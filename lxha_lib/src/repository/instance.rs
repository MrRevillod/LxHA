use mongodb::bson;
use core::ascii;
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

    pub async fn find_many_by_user_id(&self, id: &ObjectId) -> AxumResult<Vec<PublicInstance>> {
        let mut instances: Vec<PublicInstance> = vec![];
        let filter = doc! { "user_id": id };

        let mut cursor = self.collection.find(filter, None).await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)?
        ;

        while let Some(result) = cursor.try_next().await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)? {
            instances.push(PublicInstance {
                id: result.id.to_hex(),
                name: result.name,
                r#type: result.r#type,
                status: result.status,
                ip_addresses: result.ip_addresses.clone(),
                specs: result.specs.clone(),
                cluster_node: result.cluster_node,
                user_id: result.user_id.to_hex()
            });
        }

        Ok(instances)
    }

    pub async fn find(&self) -> AxumResult<Vec<PublicInstance>> {

        let mut instances: Vec<PublicInstance> = vec![];
        let mut cursor = self.collection.find(None, None).await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)?
        ;

        while let Some(result) = cursor.try_next().await
            .map_err(|_e| HttpResponse::INTERNAL_SERVER_ERROR)? {
            instances.push(PublicInstance {
                id: result.id.to_hex(),
                name: result.name,
                status: result.status,
                r#type: result.r#type,
                ip_addresses: result.ip_addresses.clone(),
                specs: result.specs.clone(),
                cluster_node: result.cluster_node,
                user_id: result.user_id.to_hex()
            });
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
