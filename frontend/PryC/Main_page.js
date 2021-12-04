import { StatusBar } from 'expo-status-bar';
import React, { Component, useEffect, useState } from 'react';
import { StyleSheet, ScrollView, Text, View, Image, useWindowDimensions, TextInput, TouchableOpacity } from 'react-native';

const ScrollWithRandomProducts = (props) => {
  //console.log(props);
  var returnElement = props.data.map((item) => {
    return (
    <TouchableOpacity style={styles.scrollableButtonElem}>
          <Image 
            style={{
              resizeMode:"cover",
              height: 70, 
              width:70,
              borderRadius:25,
            }}
            source = {{uri:'http://localhost:8000/images/' + item.photo}}
          />
    </TouchableOpacity>
    )
  });
  //console.log(returnElement);
  return (
    <>
     <View style={styles.verticalScrollableMenu}> 
     <ScrollView horizontal>
     {returnElement}
     </ScrollView>
     </View>
     </>
  );
};
/* upper scrollbar
<View style={styles.verticalScrollableMenu}>
          <ScrollView horizontal>
          <TouchableOpacity style={styles.scrollableButtonElem}>
          </TouchableOpacity>
          <TouchableOpacity style={styles.scrollableButtonElem}>
          </TouchableOpacity>
          <TouchableOpacity style={styles.scrollableButtonElem}>
          </TouchableOpacity>
          <TouchableOpacity style={styles.scrollableButtonElem}>
          </TouchableOpacity>
          <TouchableOpacity style={styles.scrollableButtonElem}>
          </TouchableOpacity>
          <TouchableOpacity style={styles.scrollableButtonElem}>
          </TouchableOpacity>
        </ScrollView>
      </View>
*/

class Main_page extends Component {
   state =  {
    randomProducts: []
  };
  
  componentDidMount() {
    (async() => {
      var request = await fetch('http://localhost:8000/json/photos');
      if (request.ok === true) {
        this.setState({randomProducts: await request.json()});
        //console.log(this.state.randomProducts);
      } else {
        console.log(`Fetch failed`);
      }
    })() 
  }

  render() {
    return (
      <View style={styles.container}>
      <View style={styles.header}>

      </View>
      <View style={styles.searchAndScan}>
        <View style={{
          width:260,
          height:55,
          backgroundColor:'#fff',
          borderWidth:1,
          justifyContent:'center',
          alignItems:'flex-end',
          borderRadius:30,
          marginTop:2.5,
          marginLeft:5,
        }}>
      <TextInput style={{width:200,
      height:45,
      backgroundColor:'#fff',
      marginRight:12,}}
      placeholder="Wyszukaj produkt"/>
      </View>
      <TouchableOpacity
        onPress={() => this.props.navigation.navigate('Scan_barcode')}
        style={{
          marginTop:5,
          marginLeft:15,
          height:50,
          width:50,
          backgroundColor:"#FFF",
          borderWidth:1,
          borderRadius:10,
        }}
        title="|||"
      />
      </View>
      <ScrollWithRandomProducts data={this.state.randomProducts}/>
      
      <View style={styles.adBanner}>

      </View>
      <View 
        style={styles.verticalScrollableMenu}
      >
        <ScrollView horizontal>
          <TouchableOpacity style={styles.scrollableButtonElem}>
          </TouchableOpacity>
          <TouchableOpacity style={styles.scrollableButtonElem}>
          </TouchableOpacity>
          <TouchableOpacity style={styles.scrollableButtonElem}>
          </TouchableOpacity>
          <TouchableOpacity style={styles.scrollableButtonElem}>
          </TouchableOpacity>
          <TouchableOpacity style={styles.scrollableButtonElem}>
          </TouchableOpacity>
          <TouchableOpacity style={styles.scrollableButtonElem}>
          </TouchableOpacity>
        </ScrollView>
      </View>
      <View style={styles.footer}>

      </View>
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
   
};


//const { height, width } = useWindowDimensions();
const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#999',
    alignItems: 'center',
    justifyContent: 'flex-start',
  },
  header: {
    backgroundColor: '#fff',
    height:50,
    width:360,
    borderBottomLeftRadius: 20,
    borderBottomRightRadius:20,
    borderWidth:1,
  },
  searchAndScan:{
    backgroundColor:'#999',
    width:340,
    height:60,
    marginTop:25,
    flexDirection: 'row',
    borderRadius:15,
  },
  scrollableButtonElem:{
    width:80,
    height:80,
    borderRadius:30,
    marginTop:10,
    marginLeft:10,
    backgroundColor:'#444',
    alignItems:'center',
    justifyContent:"center",
  },
  verticalScrollableMenu:{
    marginTop:25,
    height:100,
    width:340,
    backgroundColor:'#999',
  },
  adBanner:{
    marginTop:25,
    height:120,
    width:340,
    backgroundColor:'#444',
    borderRadius:30,
  },
  footer:{
    backgroundColor: '#fff',
    marginTop:25,
    height:85,
    width:360,
    borderTopLeftRadius: 20,
    borderTopRightRadius:20,
    borderWidth:1,
  }
});

export default Main_page;