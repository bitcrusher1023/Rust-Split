use frame_system::Module as System;
use frame_system::RawOrigin;
use frame_support::{decl_module, decl_storage, decl_event, dispatch::DispatchResult};

pub trait Trait: System {
    type Event: From<Event<Self>> + Into<<Self as System>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as Splitter {
        Recipients get(fn recipients): map hasher(blake2_128_concat) T::AccountId => u32;
        Share get(fn share): map hasher(blake2_128_concat) (T::AccountId, T::AccountId) => u32;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
    {
        FundsReceived(AccountId, u64),
        RecipientUpdated(AccountId),
        ShareUpdated(AccountId, AccountId),
        FundsSent(AccountId, u64),
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        #[weight = 10_000]
        fn receive_funds(origin, amount: u64) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            let reciver = Self::recipients();
            let total_shares: u32 = reciver.values().sum();

            for (recipient, &share) in recipients.iter() {
                let funds_to_send = amount * (share as u64) / total_shares as u64;
                Self::send_funds(sender.clone(), recipient.clone(), funds_to_send)?;
            }

            Self::deposit_event(RawEvent::FundsReceived(sender, amount));
            Ok(())
        }

        fn update_recipient(origin, recipient: T::AccountId, shares: u32) -> DispatchResult {
            let _ = ensure_root(origin)?;
            Recipients::<T>::insert(&recipient, shares);
            Self::deposit_event(RawEvent::RecipientUpdated(recipient));
            Ok(())
        }

        fn update_share(origin, recipient1: T::AccountId, recipient2: T::AccountId, share: u32) -> DispatchResult {
            let _ = ensure_root(origin)?;
            Share::<T>::insert((recipient1, recipient2), share);
            Self::deposit_event(RawEvent::ShareUpdated(recipient1, recipient2));
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    fn send_funds(sender: T::AccountId, recipient: T::AccountId, amount: u64) -> DispatchResult {
        Self::deposit_event(RawEvent::FundsSent(recipient.clone(), amount));
        Ok(())
    }
}
