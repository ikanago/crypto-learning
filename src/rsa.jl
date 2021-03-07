module Rsa

using crypto_learning.Util

function generate_key(bits::Integer)::Tuple{Tuple{BigInt, BigInt}, BigInt}
    p = Util.generate_prime(bits)
    q = Util.generate_prime(bits)
    n = p * q
    s = lcm(p - 1, q - 1)
    e = 1
    while true
        e = rand(0:s)
        if gcd(s, e) == 1
            break
        end
    end

    d = invmod(e, s)
    (e, n), d
end

function encrypt(m::BigInt, public_key::Tuple{BigInt, BigInt})::BigInt
    (e, n) = public_key
    @assert(0 <= m < n)
    c = powermod(m, e, n)
    c
end

function decrypt(ciphered::BigInt,public_key::Tuple{BigInt, BigInt}, private_key::BigInt)::BigInt
    (_, n) = public_key
    m = powermod(ciphered, private_key, n)
    @assert(m > 0)
    m
end

end
