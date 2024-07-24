# Pragma Optimistic Oracle

> This repository details the first implementation of an optimistic oracle for prediction market. 


## Project presentation

 - In the first deployed version, we will be introducing a trusted optimistic oracle. A multisig set among a list of trusted actors will be responsible for the resolution. Everyone will be able to dispute, providing a bond. Full documentation available soon.


## General graph

```mermaid
sequenceDiagram
    participant User
    participant User/PredictionMarket
    participant PragmaOO
    participant IAModule
    participant PragmaMultisig

    User->>User/PredictionMarket: Submit assertion with bond
    User/PredictionMarket->>PragmaOO: Forward assertion with bond
    PragmaOO->>PragmaOO: Generate assertion_id
    PragmaOO->>IAModule: Request prediction result
    IAModule->>IAModule: Evaluate assertion
    IAModule->>PragmaOO: Submit prediction result
    PragmaOO->>PragmaOO: Store result
    PragmaOO->>User/PredictionMarket: Notify of AI evaluation
    User/PredictionMarket->>User: Inform result and dispute window

    alt No dispute during window
        PragmaOO->>PragmaOO: Settle assertion
        PragmaOO->>User: Return bond
        PragmaOO->>User/PredictionMarket: Assertion settled
    else Dispute raised
        User->>User/PredictionMarket: Submit dispute with bond
        User/PredictionMarket->>PragmaOO: Forward dispute
        PragmaOO->>PragmaOO: Initiate dispute resolution
        Note over PragmaOO: Dispute resolution process<br>(for now it's a multisig, <br>staking will be implemented later)
        PragmaOO->>PragmaMultisig: Question the multisig
        PragmaMultisig->>PragmaOO: Display final result
        
        alt Original assertion upheld
            PragmaOO->>User: Return original bond + dispute bond (minus fees)
        else Dispute upheld
            PragmaOO->>User: Transfer both bonds to disputer (minus fees)
        end
        
        PragmaOO->>User/PredictionMarket: Final settlement result
    end

    User/PredictionMarket->>User/PredictionMarket: Update market state
    User/PredictionMarket->>User: Inform of final outcome
```


```mermaid
sequenceDiagram
    participant AuthorizedUser as Authorized User (Multisig)
    participant User
    participant Website
    participant AIAgent as AI Agent
    participant ListenerDB as Listener/DB
    participant Contract

    alt User initiates an assertion
        User->>Contract: call `assert_truth` or `assert_truth_with_default` function of <br>the contract, and send the bond to the contract
        Contract->>Contract: emit event `AssertionMade``
        ListenerDB->>Contract: listen to event emission and store event in DB
        ListenerDB->>Website: updates the website with the list of assertions(assert, dispute, resolved)
        Note right of ListenerDB: Checks on the db if the expiration <br> timestamp for a given assertion has been reached <br> or a dispute has been initiated
        alt No dispute has been made 
            ListenerDB->>Contract: call `settle_assertion` and retrieve the result
            ListenerDB->>Website: update the website status for the assertion to resolved
        else A dispute has been initiated
            ListenerDB->>Website: update the website with the assertion status associated
            Website->>AuthorizedUser: alert on dispute to resolve
        end
        ListenerDB->>Contract: 
    else Authorized user (multisig) resolves a dispute
        ListenerDB->>Contract: Monitor contract events<br>(assertions, resolutions, disputes)
        ListenerDB->>Website: Update website periodically<br>with list of assertions
        Website->>Website: Display list of disputed assertions<br>(including bond amounts)
        AuthorizedUser->>Website: Submit signed final resolution<br>for a specific assertion
        Website->>Contract: Call `settle_assertion` function<br>to finalize resolution
        Contract->>Contract: Emit settlement event
        ListenerDB->>Website: Capture event and update website
        Website->>Website: Update assertion status
        Website->>User: Notify user of new assertion status
    else AI background entity
        AIAgent->>AIAgent: listen to assertion events and checks for validity if id not stored
        alt assertion is valid
            AIAgent->>AIAgent: store the true id and move on
        else assertion is invalid
            AIAgent->>AIAgent: store the false id and start the dispute process 
            Note right of AIAgent: Another entity with funds should initiate the dispute process
        end
        AIAgent->>AIAgent: sleeps for xs
    end
```
