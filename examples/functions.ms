
function ഫാക്ടോറിയൽ(n) {
    ശെരിയാണോ n ഉം 0 ഉം സെയിം_ആണേ {
        return 1;   
    }
    return  n * ഫാക്ടോറിയൽ<n-1>;
}

ദേ_പിടിച്ചോ "Output " + ഫാക്ടോറിയൽ<5>;