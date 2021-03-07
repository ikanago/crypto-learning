using crypto_learning.Util, Test

@test Util.is_prime(1)
@test Util.is_prime(2)
@test !Util.is_prime(4)