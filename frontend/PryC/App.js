import { StatusBar } from 'expo-status-bar';
import React from 'react';
import { StyleSheet, Text, View, Image, useWindowDimensions, TextInput } from 'react-native';
export default function App() {
  return (
    <View style={styles.container}>
      <View style={styles.header}>

      </View>
      <View style={styles.searchAndScan}>
        <View style={{
          width:260,
          height:55,
          backgroundColor:'#ff6',
          justifyContent:'center',
          alignItems:'flex-end',
          borderRadius:20,
        }}>
      <TextInput style={{width:200,
      height:45,
      backgroundColor:'#fff',
      marginRight:8,}}/>
      </View>
      </View>
      <Text>Open up App.js to start working on your app!</Text>
      {/*<Image 
      source={{
        uri:'http://192.168.243.204:8000/images/IMG_20211203_135633.jpg',
      }}
      style={{width:100,height:100}}
    />*/}
      <StatusBar style="auto" />
    </View>
  );
}

//const { height, width } = useWindowDimensions();
const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    justifyContent: 'flex-start',
  },
  header: {
    backgroundColor: '#f6f',
    height:50,
    width:360,
    borderBottomLeftRadius: 20,
    borderBottomRightRadius:20,
  },
  searchAndScan:{
    backgroundColor:'#6f6',
    width:340,
    height:60,
    marginTop:25,
    flexDirection: 'row',
  }
});
