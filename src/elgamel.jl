module Elgamel

using crypto_learning.Util, Random

function generate_key(bits::Integer)::Tuple{Tuple{BigInt,BigInt,BigInt},BigInt}
    p = 0
    q = 0
    while true
        q = Util.generate_prime(bits)
        p = 2q + 1
        if Util.is_prime(p)
            break
        end
    end

    # Primitive element
    g = 0
    while true
        g = rand(3:p)
        if powermod(g, 2, p) == 1
            continue
        elseif powermod(g, q, p) == 1
            continue
        else
            break
        end
    end

    # Private Key
    x = rand(2:(p-1))
    # Public Key
    y = powermod(g, x, p)
    (p, g, y), x
end

function encrypt(m::BigInt, public_key::Tuple{BigInt,BigInt,BigInt})::Tuple{BigInt,BigInt}
    (p, g, y) = public_key
    @assert(0 <= m < p)
    r = rand(0:(p-1))
    c1 = powermod(g, r, p)
    c2 = (m * powermod(y, r, p)) % p
    c1, c2
end

function decrypt(
    ciphered::Tuple{BigInt,BigInt},
    public_key::Tuple{BigInt,BigInt,BigInt},
    private_key::BigInt,
)::BigInt
    (p, g, y) = public_key
    (c1, c2) = ciphered
    (c2 * powermod(c1, p - 1 - private_key, p)) % p
end

end
