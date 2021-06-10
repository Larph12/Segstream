<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Filter     User Working Status         _cc749d</name>
   <tag></tag>
   <elementGuidId>7677ba5d-a819-463a-8770-b81b3b85884f</elementGuidId>
   <imagePath>Screenshots/Targets/Object Repository/Page_Worksheet  SegStream/div_Filter     User Working Status         _cc749d.png</imagePath>
   <selectorCollection>
      <entry>
         <key>IMAGE</key>
         <value>Screenshots/Targets/Object Repository/Page_Worksheet  SegStream/div_Filter     User Working Status         _cc749d.png</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='rightbar']</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#rightbar</value>
      </entry>
   </selectorCollection>
   <selectorMethod>IMAGE</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>rightbar</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
            


    Filter 



    User Working Status
    
        
        Nobody is working.
        

        
    



     Information
    
        Property Name : 2500 sheet
        Tie-To Number : 
        Placed-In-Service Date : 
        
    


    Property Name : 2500 sheet
    Entity Name : test
    Project Name : Test case 1
    Tie-To Number : 
    Placed-In-Service Date : 
    Job Number : 
    
    City, State, Zipccode : Ackworth, IA, 50001
    Total Building SF : 0
    Depreciation Method : Commercial MACRS (GDS)
    Project Type : Purchase Price Allocation (No Given Costs)
    Number of Assemblies : 24
    Number of Individual Items: 16
    Total Number of Lines : 165
    

 Analysis Window


    
        
        
        
        
        
            
                
                    5 Years :
                
                0.00
                (0.00%)
            
        
        
        
            
                
                    15 Years :
                
                71,426.22
                (2.75%)
            
        
        
        
            
                
                    39 Years :
                
                2,529,702.56
                (97.25%)
            
        
        
        
        
    

    
        
            Initial Total Cost
        
        2,601,128.78
    
    
    
        
            Active Total Cost
        
        2,601,128.78
    
    
    


    
        
            
            WARNING:
                You have made changes to the Worksheet after a squeeze.
                
                Please reset the squeeze and then squeeze the sheet again when you have finished making changes by using the Reset Squeeze button and then use the Apply Squeeze button at the top of the screen to perform ALL Squeezes you want implemented again.
                
        
    








    $(&quot;#id_info_head&quot;).click(function(){
        var selectedEffect = &quot;drop&quot;;
        $(&quot;#id_info_detail_short&quot;).toggle();
        $(&quot;#id_info_detail&quot;).toggle( selectedEffect, 500, callback );
    });

    $(&quot;#id_locations_used&quot;).click(function(){
        var selectedEffect = &quot;drop&quot;;
        $(&quot;#id_locations_detail&quot;).toggle( selectedEffect, 500, callback );
    });

    function callback() {
        setTimeout(function() {
            $( &quot;#effect&quot; ).removeAttr( &quot;style&quot; ).hide().fadeIn();
        },400 );
    };

    $(document).ready(function(){
        var tmp = $('span#id_tie_to_no').html().replace(/[$]/, '');
        $('span#id_tie_to_no').html(tmp)

        $('.asset_year_in_use_links').hover(
            function(){
                $(this).addClass('add-pointer');
                $(this).closest('div').find('.asset_year_name').addClass('text-decorate');
            }, function(){
                $(this).removeClass('add-pointer');
                $(this).closest('div').find('.asset_year_name').removeClass('text-decorate');
            }
        );

        $('.asset_year_in_use_links').on('click', function(){
            $(&quot;#asset_year_in_use_clicked&quot;).val($(this).data(&quot;code&quot;));
            $('#asset_year_view_mode_form').submit();
        })
    });


        </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;rightbar&quot;)</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='rightbar']</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='EA'])[52]/following::div[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Asset Class:'])[164]/following::div[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[5]/div[2]</value>
   </webElementXpaths>
</WebElementEntity>
