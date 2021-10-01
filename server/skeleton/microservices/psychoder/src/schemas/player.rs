


/*


                                        WOLFIA FEATURES

https://medium.com/@chataks93/predicting-human-behaviour-activity-using-deep-learning-lstm-fff9030b82e7
https://www.biorxiv.org/content/biorxiv/early/2017/12/30/240317.full.pdf
https://www.youtube.com/watch?v=YrO1v7-KcXs



[!!!] audio streaming like clubhouse with low latency and bandwidth
[!!!] voice recognition for judging graph of each game
[!!!] credit score generation using position clustering and voice recognition => use core/coin_generation algos 
[!!!] player actions and talks inside the game for generating the credit
[!!!] realtime status diagram inside the game
[!!!] playing based on the level of each player and mined credit
[!!!] implement my master thesis idea for online video streaming
[!!!] a game based on MIND BEHAVIOUR ACTIVITY to deal with their fears! like black mirrors S03E02.


*/



use serde::{Serialize, Deserialize};
use std::sync::Arc;
use uuid::Uuid;






#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MetaData{
    pub id: Uuid,
    pub time: i64,
}






impl Default for MetaData{
    fn default() -> Self{
        MetaData{
            id: Uuid::new_v4(),
            time: chrono::Local::now().naive_local().timestamp(),
        }
    }
}
