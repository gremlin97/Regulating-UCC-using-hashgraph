# Regulating UCC Using DLT
Link to the previous paper: 

# Why using hasgraph over fabric
Fabric is very easy to learn and provides feature of a permissioned blockchain in a easy to use manner. It has CA authority, validates authenticity of the call parameters by itself and achieves validation using very less resources as compared to bitcoin. It has a orderer which validates and decides the sequence of transaction thus avoiding proof-of-stake and proof-of-work. 
1. Validated data still has to be replicated across each node after validation limiting scaling fabric to about 2500 calls per/sec.
2. If many parties are involved in the ecosystem, sharing data across parties has largeoverhead. Say, for some transaction they are required to share data across each other without compromising privacy; they have to make seperate channels (Bsnl-TRAI, Airtel-TRAI) to ensure privacy. Smart contracts will have to be defined for manner of data exchange.

Thus, a need for more robust, scalable and a public DLT was the best suited for achiving the ideal ecosystem.

Here, we have started researching a public, scalable dlt named hedera. Hedera is a public DLT based on hashgraph protocol. It stores data as events instead of transactions. (Suprisingly, it does not store data as blocks. Blockchain is a type of DLT!) It uses gossip-about-gossip protocol to distribut the validated events across other praticipants.


# Components
1. RTM (Registered telemarketer) Registration service
2. User preferrence and consent registration
3. Trai global registry
4. Filtering (Blocking calls not aligning to user consent)
5. TSP (Telecom Service Provider) call forwarding service
6. Audit service
7. Complaint Portal
8. Monetization (Incentivizating rule abiding nodes)

# Stack
1. Backend service: Rust, Rocket
2. DLT: Hedera Hashgraph
3. Other tools: Docker, Postman, Nginx

# Design diagram
![alt text](https://user-images.githubusercontent.com/23367724/65387692-5b05fd80-dd67-11e9-8963-c0103260ad9f.png)


# filtering-service
A robust type safe service written in rust


# Sequence flow and description
1. CP will send content and subsriber list to the IR. 
2. IR based on preferrence of users will filter the subscribers.
3. IR has to for each customer, fetch his sparse merkle tree and verify is particular category exists.
   1. smt will have bit for that pref-category set as true.
   2. Check using inclusion principle.
4. IR will generate VIDs for eligible customers against their phone number.
   1. Using shamir we will split the number into 4 shares. (shamir overcomes storing vid on DLT)
   2. At any time to access the number, there has to be present atleast >=3 customers.
   3. encrypt the key with correspoding public keys make sure requests are being sent by valid client.
   4. RTM will have all the shares. RTM after decrypting it's share will initiate the call to OAP.
   5. OAP with his decrypted key along with RTM will send the data to IR for actual number.
   6. The call to IR can be skipped, if we encrypt it with some other key like RTM or a token.

# User preferrence and consent registration

1. User preferrences are stored in sparse merkle tree.
2. Root of smt for all customers is then stored into merkle tree. 
   1. As we dont want it to changing frequently, it will be updated once a day. 
   2. Checking consistency will be very easy as root for checking inclusion and exclusion.
3. Root of merkle tree will be stored in DLT representing a trusted and verified state of the running system. 


# Incentivization/Monetization
Users complaints needs to heard and action should be taken againts the resposible parties. 
1. User can raise a complaint against a message received voilating his preference or consent. User has to sign the complaint with his certifications.
2. Complaint raised by the user is shared with the TAP. Smart contracts after verifying if delivery exists or not get invokes and makes a transisition of network state. If a user has made a false accusationhis reputation is decreased. But if on the other side complaint is accepted then a complained against RTM is raised to IR.
3. IR consructs user preferrence tree and generates a proof for exclusion for current corresponding category. If category is not being given consent user then RTM has to be penalized and reputation decreased. Otherwise, user score is decreased.

RTM should be penalised such that it occurs very rare as compared to incentivization.

# GDPR compliance
According to GDPR, any entity processing user personal data must be designed and built with consideration of the priciples and provide safeguards to protect data.

 1. Events on hashgraph will be stored after decrypting it with user and IR public keys. It can be opened when both IR and user enters their private keys.
 2. RTM will not be able to see the actual filtered list. Number validated by IR will be split into 4 (RTM, IR, OAP, TAP) using shamirs shamir secret principle. It is then encrypted using the correspoding parties public keys. This ensures to see the users actual phone number atleaast 3 out of 4 parties are neeed everytime.
 3. Hashgraph is governed by a council consisting of market leaders belonging to every existing factor. Council ensures that rules governing hashgraph keeps updating with any new affecting factor rising which can disrupt the trust of network. For example, quantum computing can break tons of systems crediting to its the fastest calculations capacity.
 4. Also, GDPR to maintain equality in the system wants that every be able to delete his data from the controlling party. Blockchain as being "immutable" does not let exercesing this write but hashgraph council can update because of their governance power.
   

# Data structures
1. merkle tree: https://hackernoon.com/merkle-tree-introduction-4c44250e2da7
2. sparse merkle tree



