use crate::tournaments::TournamentStatus;
use crate::*;

#[near_bindgen]
impl GameBloc {
    pub fn new_crowd_funded_tournament(
        &mut self,
        owner_id: AccountId,
        tournament_id_hash: String,
        game_name: String,
        no_of_users_input: U128,
        prize_input: U128,
        new_stuff:U128,
    ) {
        let _no_of_users: U128 = no_of_users_input;
        //.unwrap();
        let prize: U128 = prize_input;
        // assert_eq!(
        //     env::predecessor_account_id(),
        //     self.owner_id,
        //     "Only the owner may call this method"
        // );

        let existing = self.crowd_funded_tournaments.insert(
            &tournament_id_hash,
            &Tournament {
                owner_id,
                status: TournamentStatus::AcceptingPlayers,
                game: game_name,
                user: Vec::with_capacity(50.try_into().unwrap()),
                winers: Vec::new(),
                total_prize: prize,
            },
        );

        assert!(
            existing.is_none(),
            "Tournament with that key already exists"
        );
        self.crowd_funded_tournament_ids.insert(&tournament_id_hash);
    }

    pub fn start_crowd_funded_tournament(&mut self, crowd_funded_tournaments_id: String) -> () {
        let mut crowd_funded_tournaments = self
            .crowd_funded_tournaments
            .get(&crowd_funded_tournaments_id)
            .expect("ERR_NOT_CORRECT_USER");

        crowd_funded_tournaments.status = match crowd_funded_tournaments.status {
            TournamentStatus::AcceptingPlayers => TournamentStatus::GameInProgress,
            _ => {
                env::panic_str("ERR_GAME_IN_PROGRESS");
            }
        };

        // Reinsert the tournament back in after we modified the status:
        self.crowd_funded_tournaments
            .insert(&crowd_funded_tournaments_id, &crowd_funded_tournaments);
        crowd_funded_tournaments.status;
    }

    pub fn join_crowd_funded_tournament(
        &mut self,
        user_id: AccountId,
        crowd_funded_tournament_id: String,
    ) -> Tournament {
        let mut crowd_funded_tournament = self
            .crowd_funded_tournaments
            .get(&crowd_funded_tournament_id)
            .unwrap_or_else(|| env::panic_str("JOINING"));

        crowd_funded_tournament.user.push(user_id);
        self.crowd_funded_tournaments
            .insert(&crowd_funded_tournament_id, &crowd_funded_tournament);
        return crowd_funded_tournament;
    }

    pub fn end_crowd_funded_tournament(&mut self, crowd_funded_tournament_id: String) {
        let mut crowd_funded_tournament = self
            .crowd_funded_tournaments
            .get(&crowd_funded_tournament_id)
            .expect("ERR_NOT_CORRECT_USER");

        crowd_funded_tournament.status = match crowd_funded_tournament.status {
            TournamentStatus::GameInProgress => TournamentStatus::GameCompleted,
            _ => {
                env::panic_str("ERR_GAME_COMPLETED");
            }
        };

        // Reinsert the tournament back in after we modified the status:
        self.crowd_funded_tournaments
            .insert(&crowd_funded_tournament_id, &crowd_funded_tournament);
        crowd_funded_tournament.status;

        log!("Tournament with tournament_id hash {} completed successfully");

        // Transfer the prize money to the winner
        Promise::new(env::predecessor_account_id())
            .transfer(crowd_funded_tournament.total_prize.into());
    }
}
