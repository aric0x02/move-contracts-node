# Clone move-stdlib
rm -rf ./move-stdlib
# git clone https://github.com/pontem-network/move-stdlib ./move-stdlib
git clone -b release-v1.0.1 --single-branch https://github.com/aric0x02/move-stdlib.git
pushd ./move-stdlib
# git checkout release-v1.0.0
dove build
dove deploy
popd

# Clone pont-stdlib
rm -rf ./pont-stdlib
# git clone https://github.com/pontem-network/pont-stdlib.git ./pont-stdlib
git clone -b release-v1.0.1 --single-branch https://github.com/aric0x02/pont-stdlib.git
pushd ./pont-stdlib
# git checkout release-v1.0.0
dove build
dove deploy
popd

pushd ./user
dove clean
dove build
dove call "store_system_block()"
dove call "store_system_timestamp()"
dove call "transfer<0x1::NOX::NOX>(Alice,500000000000)"
mv ./build/assets/transaction/transfer.mvt ./build/assets/transaction/transfer_pont.mvt
#  -o=transfer_pont.mvt
dove call "transfer<0x1::KSM::KSM>(Alice,500000000000)"
mv ./build/assets/transaction/transfer.mvt ./build/assets/transaction/transfer_ksm.mvt
#  -o=transfer_ksm.mvt
dove call "deposit_bank<0x1::NOX::NOX>(500000000000)"
mv ./build/assets/transaction/deposit_bank.mvt ./build/assets/transaction/deposit_bank_pont.mvt
#  -o=deposit_bank_pont.mvt
dove call "deposit_bank<0x1::KSM::KSM>(50000000000000)" 
mv ./build/assets/transaction/deposit_bank.mvt ./build/assets/transaction/deposit_bank_ksm.mvt
# -o=deposit_bank_ksm.mvt
popd
