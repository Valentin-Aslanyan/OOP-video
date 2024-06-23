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

	int get_x()
	{
		return x;
	}
};


int main(){
	
	int i,arr_size=100000000;

	//Create array of Alice class in memory; automatically calls constructor
	Alice *Bob = new Alice[arr_size];
	int *another_array = new int[arr_size];

	//Run timing on loop over array of Alice class
	auto start = high_resolution_clock::now();
	for (i=0;i<arr_size;++i){
		another_array[i]=Bob[i].x;
	}
	auto stop = high_resolution_clock::now();
	auto duration = duration_cast<microseconds>(stop - start);
	cout << "Time for class[mus]: " << duration.count() << endl;

	//Delete array of Bob class and free memory
	delete [] another_array;
	delete [] Bob;
	return 0;

}
