# Regulating UCC Using DLT

## Abstract

Unsolicited commercial communications (UCC) means any commercial communications that is neither in accordance with consent or preference of the consumer. Though government authorities have tried to curb the issue of UCC by trying to regulate and making the ecosystem transparent. But all centralised approaches such as DND have failed to stop the UCC.

UCC has incurred users losses of around 13 billions dollars. UCC has made users to either block or not read the messages leading to useful ads even being labelled as spam. There is a need of a decentralized ecosystem that enbales a co-regulation between involved stakeholders.

## Need of a public DLT

### Problems with present ecosystem

* No way of for a content provider to verify the authenticity of an RTM. Also, as system is scattered it is next to impossible to verify and penalize an RTM in case of negligence.
  
* Moreover, as as there is not efficient auditing mechanism there is no incentive for an RTM to work fairly. 
  


### Problems with private DLT (using fabric as standard)

* Data needs to be shared across stakeholders or nodes (channels) in a secure manner. This adds a large network overhead and slows down the ecosystem.

* Due to protocols inlcuding ordering, validating etc. transactions that can be handled has a limit of about 2500 calls per/sec.

* smart contratcts once instantiated can not be changed and a minor error in them can lead to mismanagement of entire system.

* There is not option of opt out as data once recorded on DLT can not be deleted.

* there is no mechanism of token-economics involved. Stakeholder needs to be penalized and rewarded based on their action.


### Problems with Public DLT

* transactions stored on DLT can be accessed by anyone.
  
* Anyone can try to query or make a transaction overloading the DLT network.
  
### Propsed solution using hedera

* Hedera is public DLT using hashgraph consensus algorithm.
* A reputation score is associated with RTM. RTM is not given access to user data to be exploited. Score makes system transparent and CPs can negotiate better rates.

* Shamir keys

## Why using hasgraph over fabric

Fabric is very easy to learn and provides feature of a permissioned blockchain in a easy to use manner. It has CA authority, validates authenticity of the call parameters by itself and achieves validation using very less resources as compared to bitcoin. It has a orderer which validates and decides the sequence of transaction thus avoiding proof-of-stake and proof-of-work.

1. Validated data still has to be replicated across each node after validation limiting scaling fabric to about 2500 calls per/sec.
2. If many parties are involved in the ecosystem, sharing data across parties has largeoverhead. Say, for some transaction they are required to share data across each other without compromising privacy; they have to make seperate channels (Bsnl-TRAI, Airtel-TRAI) to ensure privacy. Smart contracts will have to be defined for manner of data exchange.

Thus, a need for more robust, scalable and a public DLT was the best suited for achiving the ideal ecosystem.

Here, we have started researching a public, scalable dlt named hedera. Hedera is a public DLT based on hashgraph protocol. It stores data as events instead of transactions. (Suprisingly, it does not store data as blocks. Blockchain is a type of DLT!) It uses gossip-about-gossip protocol to distribut the validated events across other praticipants.

1. It is complete asychronous, no leaders, no round robin, no proof-of-stake.
2. It validates and orders the reults with probability one.
3. It is based on gossip protocol. In hedera, participants gossip about gossip. This reduces overhead as only new information is sent to the next participant.
4. Unlike fabric, gossip about gossip allows agreement to reach without actual voting over the internet.
5. hedera being a public blockchain has a conecpt of hbar (tokens). These tokens can be used to incentivize/monetize the ecosystem serving all the stakeholders.
6. The governing council heading hedera have the power to update and modify the policies adhering to new discoveries and advancements.
7. In compliance to GDPR, council can set a mechanism to follow that for user data if stored anywhere on network.

## Components

1. RTM (Registered telemarketer) Registration service
2. User preferrence and consent registration
3. Trai global registry
4. Filtering (Blocking calls not aligning to user consent)
5. TSP (Telecom Service Provider) call forwarding service
6. Audit service
7. Complaint Portal
8. Monetization (Incentivizating rule abiding nodes)

## Stack

1. Backend service: Rust, Rocket
2. DLT: Hedera Hashgraph
3. Other tools: Docker, Postman, Nginx

## Design diagram

![alt text](https://user-images.githubusercontent.com/23367724/65387692-5b05fd80-dd67-11e9-8963-c0103260ad9f.png)


## filtering-service

A robust type safe service written in rust. Postman API documentation: [Postman Link](https://documenter.getpostman.com/view/2319897/SVtbQ5aG?version=latest)


## Sequence flow and description

1. A message will be initiated by Content provider. CP will request filtering service to handle the validations.
2. Filtering service will check for consent. Instead of validating the preference, it will done by IR.
3. RTM will send subsriber list to the IR. 
4. IR based on preferrence of users will filter the subscribers.
5. IR has to for each customer, fetch his sparse merkle tree and verify is particular category exists.
   1. smt will have bit for that pref-category set as true.
   2. Check using inclusion principle.
6. IR will generate VIDs for eligible customers against their phone number.
   1. Using shamir we will split the number into 4 shares. (shamir overcomes storing vid on DLT)
   2. At any time to access the number, there has to be present atleast >=3 customers.
   3. encrypt the key with correspoding public keys make sure requests are being sent by valid client.
   4. RTM will have all the shares. RTM after decrypting it's share will initiate the call to OAP.
   5. OAP with his decrypted key along with RTM will send the data to IR for actual number.
   6. The call to IR can be skipped, if we encrypt it with some other key like RTM or a token.
   
## Sequence flow diagram

![alt text](https://user-images.githubusercontent.com/23367724/67154904-26af3e00-f321-11e9-979b-394b508cfdbc.png)

## User preferrence and consent registration

1. User preferrences are stored in sparse merkle tree.
2. Root of smt for all customers is then stored into merkle tree. 
   1. As we dont want it to changing frequently, it will be updated once a day. 
   2. Checking consistency will be very easy as root for checking inclusion and exclusion.
3. Root of merkle tree will be stored in DLT representing a trusted and verified state of the running system. 


## Incentivization/Monetization

Users complaints needs to heard and action should be taken againts the resposible parties.

1. User can raise a complaint against a message received voilating his preference or consent. User has to sign the complaint with his certifications.
2. Complaint raised by the user is shared with the TAP. Smart contracts after verifying if delivery exists or not get invokes and makes a transisition of network state. If a user has made a false accusationhis reputation is decreased. But if on the other side complaint is accepted then a complained against RTM is raised to IR.
3. IR consructs user preferrence tree and generates a proof for exclusion for current corresponding category. If category is not being given consent user then RTM has to be penalized and reputation decreased. Otherwise, user score is decreased.

RTM should be penalised such that it occurs very rare as compared to incentivization.

RTM can be penalized in below mentioned cases:

 1. It has sent a message not adhering to the mentioned category.
 2. It has used another template.
 3. It has again registered a new number and is repeating the above tasks.


RTM should be incentivized: 

 1. It has followed and was fair throughout the process.
 2. Didnt carried the UTM messages.

Customer preferences change reflects on global registry after a week. Penalization and incentivization should be calculated after the update has been done. IR will run a proof generation for the open complaints and will penalize the stakeholders after verification. 

## GDPR compliance

According to GDPR, any entity processing user personal data must be designed and built with consideration of the priciples and provide safeguards to protect data.

 1. Events on hashgraph will be stored after decrypting it with user and IR public keys. It can be opened when both IR and user enters their private keys.
 2. RTM will not be able to see the actual filtered list. Number validated by IR will be split into 4 (RTM, IR, OAP, TAP) using shamirs shamir secret principle. It is then encrypted using the correspoding parties public keys. This ensures to see the users actual phone number atleaast 3 out of 4 parties are neeed everytime.
 3. Hashgraph is governed by a council consisting of market leaders belonging to every existing factor. Council ensures that rules governing hashgraph keeps updating with any new affecting factor rising which can disrupt the trust of network. For example, quantum computing can break tons of systems crediting to its the fastest calculations capacity.
 4. Also, GDPR to maintain equality in the system wants that every be able to delete his data from the controlling party. Blockchain as being "immutable" does not let exercesing this write but hashgraph council can update because of their governance power.

## Smart Contracts

Hedera uses the same compiled solidity code. 
Byte code needs to be stored as a file on hedera network node. Charges are charged based on certain parameters like network charges, data size etc. A file id is returned to be used for future reference. These files can be queried from a single node reducing the transaction cost.</br>

Smart contracts can be created and called by assigning the file id of the byte code stored. </br>

Thoughts about smart contracts related to UCC case.

1. IR is responsible for auditing here. IR has to have the history of when the number was accessed by any profiting stakeholder. Here, RTM, OAP and TAP are the parties benifittiing from the mentioned scenario. RTM share has to be accessed for getting the actual decrypted number. IR has to contacted before accessing any number. That time, we can have a smart contract ensuring the parties validity and storing the interaction as a transaction.
2. Consistency of merkle tree storing the user preference snapshot root. As change in preferences is not dynamic, it has to be checked for consistency with respect to new state. Prervious existing tree root has to be a child root of the new tree root version.
3. Should we be storing the template?
4. Also, we have to penalize the RTM to let pass or creating spam calls. Smart contracts will check the RTM behaviour in the ecosystem. Based on the performed behaaviour penalization or profit will be calculated and imposed via smart contract.

## merkle tree thoughts

1. Index length for the module represents 
   
    |------------------------bytes---------------------------| </br>
    |------[index_length]-------|---[bytes.len()-index_length]---| </br>
    |--  used for leaf position |      used for leaf hash        | </br>

2. index_length can be though of fixing a particular category. Example, index_length = 30 includes a category : "1", mode : "1" . This will remain fix referring to fixation of categories.

3. bytes.len()-index_length is the data to be stored. This can be a binary representation for index_length. Logically,  each leaf represents ( index_length : index_data ) collectively.


## FAQS

q1. Is vid generated same for number after first time? <br>
q2. Does trx of oap to tap also be included? <br>
q3. does accessing of data for pref also should be recorded? <br>
q3. Should message be also encrypted? <br>

## Data structures

1. [merkle tree](https://hackernoon.com/merkle-tree-introduction-4c44250e2da7)
2. [Hedera hashgraph whitepaper](https://www.hedera.com/whitepaper)
3. [Bat monetizing attention](https://basicattentiontoken.org/BasicAttentionTokenWhitePaper-4.pdf)