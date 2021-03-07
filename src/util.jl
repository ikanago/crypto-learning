module Util

function is_prime(n)
    i = 2
    while i * i <= n
        if n % i == 0
            return false
        end
    end
    true
end

end