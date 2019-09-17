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

# Storing user pref and consent
1. User preferrences are stored in sparse merkle tree.
2. Root of smt for all customers is then stored into merkle tree. 
   1. As we dont want it to changing frequently, it will be updated once a day. 
   2. Checking consistency will be very easy as root for checking inclusion and exclusion.
3. Root of merkle tree will be stored in DLT representing a trusted and verified state of the running system. 

# Data structures
1. merkle tree
2. sparse merkle tree



