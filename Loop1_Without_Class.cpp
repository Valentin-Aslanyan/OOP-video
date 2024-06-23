#include <chrono>
#include <iostream>
using namespace std;
using namespace std::chrono;


class Alice{
public:
	int x;
	char y;
	
	Alice(){
		x=0;
		y='a';
	}
};


int main(){
	
	int i,arr_size=100000000;

	//Create separate arrays of x,y in memory and loop to initialize
	int *x = new int[arr_size];
	char *y = new char[arr_size];
	for (i=0;i<arr_size;++i){
		x[i]=0;
		y[i]='a';
	}

	//Run timing on loop over separate arrays
	auto start = high_resolution_clock::now();
	for (i=0;i<arr_size;++i){
		x[i]=i;
		y[i]+=(i%10);
	}
	auto stop = high_resolution_clock::now();
	auto duration = duration_cast<microseconds>(stop - start);
	cout << "Time for separate[mus]: " << duration.count() << endl;

	//Delete separate arrays and free memory
	delete [] x;
	delete [] y;
	return 0;

}
