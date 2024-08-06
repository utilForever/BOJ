//; # \u000a import java.util.Scanner; class Main { public static void main(String[] args) { Scanner scanner = new Scanner(System.in); int[] vals = { 0, 1, 10, 11, 100, 101, 110, 111, 1000, 1001, 1010 }; int n = scanner.nextInt(); for (int i = 0; i <= n; ++i) System.out.print(vals[i]); } } /*
#if true /*
vals = [ 0, 1, 10, 11, 100, 101, 110, 111, 1000, 1001, 1010 ]; n = gets.chomp().to_i; (0..n).each{ |i| print vals[i] }
=begin
/* */ // \u000a /*
#include <iostream>
int main() { int vals[] = { 0, 1, 10, 11, 100, 101, 110, 111, 1000, 1001, 1010 }; int n; std::cin >> n; for (int i = 0; i <= n; ++i) std::cout << vals[i]; return 0; }
// */
// */ let vals = [ 0, 1, 10, 11, 100, 101, 110, 111, 1000, 1001, 1010 ]; let n = Int(readLine()!); for i in 0...n! { print(vals[i], terminator: "") } // \u000a /*
#endif
/*
=end
# */