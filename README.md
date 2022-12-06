# CarleChain
__Group Name:__ BigBlock

__Group:__ Ayush (akhot2), Jason (jasonoh3), Luke (lukep2), Sam (samuel37)

__Project Introduction:__ Our goal for this project is to build a decentralized blockchain for managing personal information in a hospital setting. If we have additional time, we can implement a logistic regression algorithm to find patterns in death based on sex, age, pre-existing conditions, and other health factors. We've chosen to work on this project because it's a large area of research and may prove useful in identifying health patterns while ensuring personal information isn't public.

---

__Technical Overview:__ 

 - Create blockchain data structure for holding information

 - Implement logistic regression algorithm to predict death from patient health
 
__Checkpoint 1:__
 
 - Use existing "dummy" personal information from Hospital data to simulate decentralized blockchain in-use

 - Implement data structure for a "block" and "blockchain"

 - Implement nonce, hash, and mining algorithm
 
__Checkpoint 2:__

 - Implement logisitic regression algorithm on blockchain to learn patterns in death using patient health information

 - Implement multi-threading for reading CSV patient files (simulate multiple hospitals sending in patient information)
 
__Possible Challenges:__

 - Creating blockchain that does not conflict and has multiple ids and hash iterations
 
 - Creating blockchain and logistic regression algorithms in Rust

---

__References:__

[Example Dataset](https://www.kaggle.com/datasets/tanmoyx/covid19-patient-precondition-dataset)

[Blockchain Example](https://blog.logrocket.com/how-to-build-a-blockchain-in-rust/)

[Logistic Regression Example](https://paulkernfeld.com/2018/07/01/logistic-regression-in-rust.html)