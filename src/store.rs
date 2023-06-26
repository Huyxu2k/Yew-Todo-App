use serde::{Deserialize,Serialize};
use yewdux::{prelude::*, dispatch};
use uuid::Uuid;

#[derive(Debug,Deserialize,Serialize,PartialEq,Clone)]
pub struct Feedback{
    pub id:Uuid,
    pub text:String,
    pub rating:u8,
}
#[derive(Debug,PartialEq,Serialize,Deserialize,Clone,Default)]
pub struct  AlertInput{
    pub show_alert:bool,
    pub alert_message:String,
}
#[derive(Default,PartialEq,Deserialize,Serialize,Store,Clone)]
#[store(storage="local",storage_tab_sync)]
pub struct Store{
    pub feebacks:Vec<Feedback>,
    pub loading:bool,
    pub alert_input:AlertInput,
}
pub fn set_feedback(feedback:Feedback,dispatch:Dispatch<Store>){
    dispatch.reduce_mut(move |store|{
        store.feebacks.insert(0, feedback);
    })
}
pub fn delete_feedback(id:Uuid,dispatch:Dispatch<Store>){
    dispatch.reduce_mut(move|store|{
        store.feebacks.retain(|f|f.id !=id);
    })
}
pub fn set_loading(loading:bool,dispatch:Dispatch<Store>){
    dispatch.reduce_mut(move|store|{
        store.loading=loading;
    })
}
pub fn set_show_alert(message:String,dispatch: Dispatch<Store>){
    dispatch.reduce_mut(move|store|{
        store.alert_input=AlertInput{
            alert_message:message,
            show_alert:true
        }
    })
}
pub fn set_hide_alert(dispatch:Dispatch<Store>){
    dispatch.reduce_mut(move|store|{
        store.alert_input.show_alert=false;
    })
}