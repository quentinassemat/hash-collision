# hash-collision
Small collision (few bytes) in shake 128 with various algorithm (including Rho, Oorschot Wiener)

I first tried to compute collision with non personalized collisions. (result in the file non personalized). I first used a brute force algorithm and then a van Oorschot and Weiner algorithm. I tried to choose the number of trailing zeros according to this https://mtrimoska.com/slides/Time_memory_analysis_for_PCS.pdf to minimize the time complexity. 

Then I have personalized my collision. The idea is to mimic a passphrase (for example of a crypto currency hardware wallet). I then changed both my algorithm to find personalized collision. 

runtime for 8 bytes collision : a few hours on my old computer
