#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, token, Address, Env, String,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Campaign {
    pub owner: Address,
    pub token: Address,
    pub title: String,
    pub total_donated: i128,
    pub total_withdrawn: i128,
    pub active: bool,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    NextCampaignId,
    Campaign(u32),
    DonatedByUser(u32, Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum DonationError {
    CampaignNotFound = 1,
    InvalidAmount = 2,
    CampaignClosed = 3,
    InsufficientCampaignBalance = 4,
}

#[contract]
pub struct DonationBox;

#[contractimpl]
impl DonationBox {
    pub fn create_campaign(
        env: Env,
        owner: Address,
        token: Address,
        title: String,
    ) -> Result<u32, DonationError> {
        owner.require_auth();

        let campaign_id = Self::next_campaign_id(&env);

        let campaign = Campaign {
            owner: owner.clone(),
            token,
            title,
            total_donated: 0,
            total_withdrawn: 0,
            active: true,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Campaign(campaign_id), &campaign);

        env.storage()
            .instance()
            .set(&DataKey::NextCampaignId, &(campaign_id + 1));

        env.events().publish(
            (symbol_short!("created"), campaign_id),
            owner,
        );

        Ok(campaign_id)
    }

    pub fn donate(
        env: Env,
        campaign_id: u32,
        donor: Address,
        amount: i128,
    ) -> Result<(), DonationError> {
        donor.require_auth();

        if amount <= 0 {
            return Err(DonationError::InvalidAmount);
        }

        let mut campaign = Self::read_campaign(&env, campaign_id)?;

        if !campaign.active {
            return Err(DonationError::CampaignClosed);
        }

        let contract_address = env.current_contract_address();
        let token_client = token::Client::new(&env, &campaign.token);

        token_client.transfer(&donor, &contract_address, &amount);

        campaign.total_donated += amount;

        env.storage()
            .persistent()
            .set(&DataKey::Campaign(campaign_id), &campaign);

        let donated_key = DataKey::DonatedByUser(campaign_id, donor.clone());

        let previous_amount: i128 = env
            .storage()
            .persistent()
            .get(&donated_key)
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&donated_key, &(previous_amount + amount));

        env.events().publish(
            (symbol_short!("donated"), campaign_id),
            (donor, amount),
        );

        Ok(())
    }

    pub fn withdraw(
        env: Env,
        campaign_id: u32,
        to: Address,
        amount: i128,
    ) -> Result<(), DonationError> {
        if amount <= 0 {
            return Err(DonationError::InvalidAmount);
        }

        let mut campaign = Self::read_campaign(&env, campaign_id)?;

        campaign.owner.require_auth();

        let available = campaign.total_donated - campaign.total_withdrawn;

        if amount > available {
            return Err(DonationError::InsufficientCampaignBalance);
        }

        let contract_address = env.current_contract_address();
        let token_client = token::Client::new(&env, &campaign.token);

        token_client.transfer(&contract_address, &to, &amount);

        campaign.total_withdrawn += amount;

        env.storage()
            .persistent()
            .set(&DataKey::Campaign(campaign_id), &campaign);

        env.events().publish(
            (symbol_short!("withdraw"), campaign_id),
            (to, amount),
        );

        Ok(())
    }

    pub fn close_campaign(env: Env, campaign_id: u32) -> Result<(), DonationError> {
        let mut campaign = Self::read_campaign(&env, campaign_id)?;

        campaign.owner.require_auth();

        campaign.active = false;

        env.storage()
            .persistent()
            .set(&DataKey::Campaign(campaign_id), &campaign);

        env.events().publish(
            (symbol_short!("closed"), campaign_id),
            campaign.owner,
        );

        Ok(())
    }

    pub fn get_campaign(env: Env, campaign_id: u32) -> Result<Campaign, DonationError> {
        Self::read_campaign(&env, campaign_id)
    }

    pub fn get_donated_by_user(env: Env, campaign_id: u32, user: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::DonatedByUser(campaign_id, user))
            .unwrap_or(0)
    }

    pub fn get_available_balance(env: Env, campaign_id: u32) -> Result<i128, DonationError> {
        let campaign = Self::read_campaign(&env, campaign_id)?;
        Ok(campaign.total_donated - campaign.total_withdrawn)
    }

    fn read_campaign(env: &Env, campaign_id: u32) -> Result<Campaign, DonationError> {
        env.storage()
            .persistent()
            .get(&DataKey::Campaign(campaign_id))
            .ok_or(DonationError::CampaignNotFound)
    }

    fn next_campaign_id(env: &Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::NextCampaignId)
            .unwrap_or(0)
    }
}