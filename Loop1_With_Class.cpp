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

	//Create array of Alice class in memory; automatically calls constructor
	Alice *Bob = new Alice[arr_size];

	//Run timing on loop over array of Alice class
	auto start = high_resolution_clock::now();
	for (i=0;i<arr_size;++i){
		Bob[i].x=i;
		Bob[i].y+=(i%10);
	}
	auto stop = high_resolution_clock::now();
	auto duration = duration_cast<microseconds>(stop - start);
	cout << "Time for class[mus]: " << duration.count() << endl;

	//Delete array of Bob class and free memory
	delete [] Bob;
	return 0;
}
